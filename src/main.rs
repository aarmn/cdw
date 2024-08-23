use std::env;
use std::fs;
use std::process::Command;
use std::io::Write;
use clap::Error;
use std::str::FromStr;
use strum_macros::{EnumString, Display};
use clap::{Command as ClapCommand, Arg, ArgAction};

#[derive(Debug, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
enum Shell {
    Bash,
    Zsh,
    Fish,
    #[strum(serialize = "pwsh")]
    #[strum(serialize = "powershell")]
    PowerShell,
    #[strum(serialize = "nu")]
    #[strum(serialize = "nushell")]
    Nushell,
    Xonsh,
    Ksh,
    Sh,
}

fn main() {
    let mut cmd = ClapCommand::new("cdw")
        .version("1.0")
        .author("Aryan L. Horizon (AARMN) <aarmn80@gmail.com>")
        .about("Change directory to a Windows path in WSL with ease")
        .arg(Arg::new("init")
            .short('i')
            .long("init")
            .help("Initialize shell function")
            .action(ArgAction::SetTrue))
        .arg(Arg::new("init-display")
            .long("init-display")
            .help("Display shell function")
            .value_name("SHELL"))
        .arg(Arg::new("init-all")
            .long("init-all")
            .help("Initialize shell function for all available shells")
            .action(ArgAction::SetTrue))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Enable verbose mode")
            .action(ArgAction::SetTrue))
        .arg(Arg::new("convert")
            .short('c')
            .long("convert")
            .help("Convert path without changing directory")
            .action(ArgAction::SetTrue))
        .arg(Arg::new("path")
            .help("Windows path to change directory to")
            .index(1));
        
    let matches = cmd.try_get_matches_from_mut(std::env::args_os())
    .unwrap_or_else(|e| e.exit());

    /////////////////////
    // process matches //
    /////////////////////

    let verbose = matches.get_flag("verbose");
    let convert = matches.get_flag("convert");

    if matches.get_flag("init") {
        let shell = detect_shell();
        init_shell(&shell, false);
    } else if matches.get_flag("init-all") {
        let available = available_shells();
        for shell in available {
            let shell_str = &shell.to_string();
            init_shell(&shell_str, true);
        }
    } else if matches.contains_id("init-display") {
    	let fallback_shell = detect_shell();
        let shell = matches
            .get_one::<String>("init-display")
            .map(|s| s.as_str())
            .unwrap_or_else(|| fallback_shell.as_str());
        println!("{}", get_shell_function(shell));
    } else if let Some(windows_path) = matches.get_one::<String>("path") {
        let wsl_path = windows_to_wsl_path(windows_path);
        if convert {
            println!("{}", wsl_path);
        } else {
            println!("\u{0007}{}", wsl_path);
        }
        if verbose {
            println!("Windows path: {}", windows_path);
            println!("WSL path: {}", wsl_path);
        }
    } else {
        cmd.print_help().expect("Failed to print help message");
        std::process::exit(1);
    }
}

fn available_shells() -> Vec<Shell> {
    let mut shells = Vec::new();

    if Command::new("bash").arg("--version").output().is_ok() {
        shells.push(Shell::Bash);
    }
    if Command::new("zsh").arg("--version").output().is_ok() {
        shells.push(Shell::Zsh);
    }
    if Command::new("fish").arg("--version").output().is_ok() {
        shells.push(Shell::Fish);
    }
    if Command::new("pwsh").arg("-Version").output().is_ok() {
        shells.push(Shell::PowerShell);
    }
    if Command::new("nu").arg("--version").output().is_ok() {
        shells.push(Shell::Nushell);
    }
    if Command::new("xonsh").arg("--version").output().is_ok() {
        shells.push(Shell::Xonsh);
    }
    if Command::new("ksh").arg("--version").output().is_ok() {
        shells.push(Shell::Ksh);
    }
    
    // Check for sh using `sh -c "echo 1"`
    if let Ok(output) = Command::new("sh").arg("-c").arg("echo 1").output() {
        if let Ok(output_str) = std::str::from_utf8(&output.stdout) {
            if output_str.trim() == "1" {
                shells.push(Shell::Sh);
            }
        }
    }

    shells
}

fn detect_shell_by_parent_process() -> Result<String, String> {
    // Get the parent process ID (PPID) of the current shell
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("ps -p $$ -o ppid=")
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        let ppid = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // Get the grandparent process ID (PPID of the parent process)
        let parent_output = std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("ps -p {} -o ppid=", ppid))
            .output()
            .map_err(|e| format!("Failed to get grandparent process ID: {}", e))?;

        if parent_output.status.success() {
            let grandparent_pid = String::from_utf8_lossy(&parent_output.stdout).trim().to_string();

            // Get the name of the grandparent process
            let grandparent_name_output = std::process::Command::new("sh")
                .arg("-c")
                .arg(format!("ps -p {} -o comm=", grandparent_pid))
                .output()
                .map_err(|e| format!("Failed to get grandparent process name: {}", e))?;

            if grandparent_name_output.status.success() {
                let grandparent_name = String::from_utf8_lossy(&grandparent_name_output.stdout).trim().to_string();
                Ok(grandparent_name)
            } else {
                Err("Error executing command to get grandparent process name".to_string())
            }
        } else {
            Err("Error executing command to get grandparent process ID".to_string())
        }
    } else {
        Err("Error executing command to get parent process ID".to_string())
    }
}

fn detect_shell() -> String {

    if let Ok(parent_process) = env::var("PSModulePath") {
        if parent_process.to_lowercase().contains("powershell") {
            return "pwsh".to_string();
        }
    }

    let sh = Shell::from_str(
        detect_shell_by_parent_process()
        .unwrap_or_default()
        .as_str()
    );

    if let Ok(shell) = sh {
        return shell.to_string();
    }
    
    if env::var("XONSH_VERSION").is_ok() {
        return "xonsh".to_string();
    }
    if env::var("NU_VERSION").is_ok() {
        return "nu".to_string();
    }
    // Check for fish-specific environment variable
    if std::env::var("FISH_VERSION").is_ok() {
        return "fish".to_string();
    }
    // Check for other shell-specific environment variables
    if std::env::var("ZSH_VERSION").is_ok() {
        return "zsh".to_string();
    }
    if std::env::var("BASH_VERSION").is_ok() {
        return "bash".to_string();
    }  

    return "sh".to_string();

}

fn get_shell_function(shell: &str) -> String {

// couldn't cd error

    match shell {
        "fish" => r#"
function cdw
    set cmd_output (command cdw $argv | string escape)
    set exit_status $status
    if test $exit_status -eq 0
        if string match -q "\cg*" -- "$cmd_output"
            cd (string sub -s 4 -- "$cmd_output")
        else
            command cdw $argv
        end
    else
        command cdw $argv
        return $exit_status
    end
end

# Override the help function to preserve formatting
function __fish_cdw_help
    command cdw --help
end

# Set up completion to use the custom help function
complete -c cdw -f -a "(__fish_cdw_help | string match -r '^  [^ ].*')"   
"#,
        "zsh" | "bash" | "sh" | "ksh" => r#"
cdw() {
    local cmd_output exit_status
    cmd_output=$(command cdw "$@")
    exit_status=$?
    if [ $exit_status -eq 0 ]; then
        first_char=$(echo -n "$cmd_output" | cut -c1)
        if [ "$(printf '%d' "'$first_char")" -eq 7 ]; then
            cd "${cmd_output#?}"
        else
            command cdw "$@"
        fi
    else
        command cdw "$@"
        return $exit_status
    fi
}
"#,
        "xonsh" => r#"
def cdw(args):
    import subprocess
    try:
        cmd_output = subprocess.check_output(['cdw'] + args, text=True)
        if cmd_output.startswith('\a'):
            %cd @(cmd_output[1:].strip())
        else:
            print(cmd_output, end='')
    except subprocess.CalledProcessError as e:
        print(e.output, end='')
        return e.returncode
"#,
        "nushell" => r#"#!/usr/bin/env nu
        
def --wrapped --env cdw [...args: string] {
    let cmd_output = ^cdw ...$args
    if ($env.LAST_EXIT_CODE == 0) and ($cmd_output | str starts-with "\u{7}") {
        cd ($cmd_output | str substring 1..)
    } else {
        print $cmd_output
    }
}
"#,
        "pwsh" => r#"#!/usr/bin/env pwsh

function cdw {
    $cmd_output =  & (Get-Command -Name cdw -CommandType Application).Definition[0] $args
    if ($LASTEXITCODE -eq 0) {
        if ($cmd_output.StartsWith("`a")) {
            Set-Location $cmd_output.Substring(1)
        } else {
            $cmd_output
        }
    } else {
        $cmd_output
        $LASTEXITCODE = 1
    }
}
"#,
        _ => "# Unsupported shell\n",
    }.to_string()
}

fn init_shell(shell: &str, init_all_mode: bool) {
    let function = get_shell_function(&shell);
    append_to_shell_config(&shell, &function);
    println!("Function added to your {} configuration.", shell);
    println!("{}Run `{}` in your terminal to apply the changes", 
        if init_all_mode {format!("In {} shell, ", &shell)} else {"".to_string()},
        get_user_source_line(shell));
}

fn append_to_shell_config(shell: &str, function: &str) {
    let home = env::var("HOME").expect("HOME not set");
    let cdw_config_dir = format!("{}/.config/cdw", home);
    fs::create_dir_all(&cdw_config_dir).expect("Failed to create cdw config directory");

    let appendix = if shell == "nushell" {
        "nu"
    } else if "pwsh" == shell {
        "ps1"
    }
    else {
        shell
    };
    let function_file = format!("{}/function.{}", cdw_config_dir, appendix);
    let autocomplete_file = format!("{}/autocomplete.{}", cdw_config_dir, appendix);

    fs::write(&function_file, function).expect("Failed to write function file");
    fs::write(&autocomplete_file, get_autocomplete_script(shell)).expect("Failed to write autocomplete file");

    let config_file = get_shell_config_file(shell, &home);
    ensure_config_file_exists(&config_file);

    let source_line = get_source_line(shell, &function_file, &autocomplete_file);

    if !file_contains(&config_file, &source_line) {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&config_file)
            .expect("Failed to open config file");

        writeln!(file, "\n# Added by cdw\n{}", source_line)
            .expect("Failed to write to config file");
    }
}

fn ensure_config_file_exists(config_file: &str) {
    let path = std::path::Path::new(config_file);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create parent directories");
    }
    if !path.exists() {
        std::fs::File::create(path).expect("Failed to create config file");
    }
}

fn get_shell_config_file(shell: &str, home: &str) -> String {
    match shell {
        "fish" => format!("{}/.config/fish/config.fish", home),
        "zsh" => format!("{}/.zshrc", home),
        "bash" => format!("{}/.bashrc", home),
        "ksh" => format!("{}/.kshrc", home),
        "xonsh" => format!("{}/.xonshrc", home),
        "nushell" => format!("{}/.config/nushell/config.nu", home),
        "pwsh" => format!("{}/.config/powershell/Microsoft.PowerShell_profile.ps1", home),
        "sh" => format!("{}/.profile", home),
        _ => format!("{}/.profile", home),
    }
}

fn windows_to_wsl_path(path: &str) -> String {
    if path.chars().nth(1) == Some(':') &&
       path.chars().nth(0).map(|c| c.to_ascii_uppercase()).filter(|&c| c >= 'A' && c <= 'Z').is_some() &&
       path.chars().nth(2) == Some('\\')
    {
    let drive_letter = path.chars().next().unwrap().to_lowercase();
    let converted_path = path[3..].replace("\\", "/");
        format!("/mnt/{}/{}", drive_letter, converted_path)
    } 
    else if path.chars().nth(1) == Some(':') &&
            path.chars().nth(0).map(|c| c.to_ascii_uppercase()).filter(|&c| c >= 'A' && c <= 'Z').is_some() &&
            path.len() == 2
    {
        let drive_letter = path.chars().next().unwrap().to_lowercase();
        format!("/mnt/{}/", drive_letter)
    }
    else {
        format!("{}", path)
    }
}

fn get_source_line(shell: &str, function_file: &str, autocomplete_file: &str) -> String {    
    match shell {
        "fish" => format!("source {}; source {}", function_file, autocomplete_file),
        // "nushell" => format!("source {}; source {}", function_file, autocomplete_file),
        "nushell" => format!("source {}", function_file),
        // "pwsh" => format!(". {}; . {}", function_file, autocomplete_file),
        "pwsh" => format!(". {}", function_file),
        // _ => format!("[ -f {} ] && . {}; [ -f {} ] && . {}", function_file, function_file, autocomplete_file, autocomplete_file),
        _ => format!(". {}", function_file),
    }
}

fn get_user_source_line(shell: &str) -> String {
    let home = env::var("HOME").expect("HOME not set");
    let file = get_shell_config_file(shell, home.as_str());
    format!("source {}", file)
}

fn file_contains(file_path: &str, content: &str) -> bool {
    if let Ok(file_content) = fs::read_to_string(file_path) {
        file_content.contains(content)
    } else {
        false
    }
}

fn get_autocomplete_script(shell: &str) -> String {
    match shell {
        "zsh" => r#"
#compdef cdw

_cdw() {
    local curcontext="$curcontext" state line
    typeset -A opt_args

    _arguments -C \
        '-i[Initialize shell function]' \
        '--init[Initialize shell function]' \
        '-v[Enable verbose mode]' \
        '--verbose[Enable verbose mode]' \
        '-c[Convert path without changing directory]' \
        '--convert[Convert path without changing directory]' \
        '--init-all[Initialize shell function for all available shells]' \
        '--init-display[Display shell function]:shell:(bash zsh fish pwsh nushell xonsh ksh sh)' \
        '*:filename:_files'
}

compdef _cdw cdw
"#,
        "bash" => r#"
_cdw_autocomplete() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    opts="-i --init -v --verbose -c --convert --init-all --init-display"

    if [[ ${cur} == -* ]] ; then
        COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
        return 0
    fi

    if [[ ${prev} == "--init-display" ]] ; then
        COMPREPLY=( $(compgen -W "bash zsh fish pwsh nushell xonsh ksh sh" -- ${cur}) )
        return 0
    fi

    COMPREPLY=( $(compgen -f ${cur}) )
    return 0
}
complete -F _cdw_autocomplete cdw
"#,
        "fish" => r#"
function _cdw_autocomplete
    set -l cmd (commandline -opc)
    set -l cur (commandline -ct)
    set -l opts -i --init -v --verbose -c --convert --init-all --init-display

    if string match -q -- '-*' $cur
        printf '%s\n' $opts
    else if test (count $cmd) -gt 1; and test "$cmd[-1]" = "--init-display"
        printf '%s\n' bash zsh fish pwsh nushell xonsh ksh sh
    else
        __fish_complete_path $cur
    end
end

complete -f -c cdw -a '(_cdw_autocomplete)'
"#,
        "pwsh" => r#"
Register-ArgumentCompleter -Native -CommandName cdw -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)
    $opts = @('-i', '--init', '-v', '--verbose', '-c', '--convert', '--init-all', '--init-display')
    $shells = @('bash', 'zsh', 'fish', 'pwsh', 'nushell', 'xonsh', 'ksh', 'sh')

    if ($wordToComplete -match '^-') {
        return $opts | Where-Object { $_ -like "$wordToComplete*" }
    }

    if ($commandAst.CommandElements.Count -gt 2 -and $commandAst.CommandElements[-2] -eq '--init-display') {
        return $shells | Where-Object { $_ -like "$wordToComplete*" }
    }

    return Get-ChildItem -Path $wordToComplete* | Select-Object -ExpandProperty FullName
}
"#,
"nushell" => r#"
def "nu-complete cdw" [] {
    let opts = ['-i' '--init' '-v' '--verbose' '-c' '--convert' '--init-all' '--init-display']
    let shells = ['bash' 'zsh' 'fish' 'pwsh' 'nushell' 'xonsh' 'ksh' 'sh']
    let input = ($in | str trim)
    
    if ($input | str starts-with '-') {
        $opts | where { $it | str starts-with $input }
    } else if ($input == '' or $input == '--init-display') {
        $shells
    } else {
        ls ($input + '*') | get name
    }
}

def "cdw completions" [] {
    [
        {name: 'path', type: 'string', description: 'Windows path to change directory to', template: 'nu-complete cdw'},
        {name: '--init', shorthand: '-i', type: 'switch', description: 'Initialize shell function'},
        {name: '--init-all', type: 'switch', description: 'Initialize shell function for all available shells'},
        {name: '--init-display', type: 'string', description: 'Display shell function', template: 'nu-complete cdw'},
        {name: '--verbose', shorthand: '-v', type: 'switch', description: 'Enable verbose mode'},
        {name: '--convert', shorthand: '-c', type: 'switch', description: 'Convert path without changing directory'}
    ]
}

export extern cdw [...args: string@'cdw completions']
"#,
        "xonsh" => r#"
def _cdw_completer(prefix, line, begidx, endidx, ctx):
    opts = ['-i', '--init', '-v', '--verbose', '-c', '--convert', '--init-all', '--init-display']
    shells = ['bash', 'zsh', 'fish', 'pwsh', 'nushell', 'xonsh', 'ksh', 'sh']

    if prefix.startswith('-'):
        return [o for o in opts if o.startswith(prefix)]
    elif len(line.split()) > 2 and line.split()[-2] == '--init-display':
        return [s for s in shells if s.startswith(prefix)]
    else:
        return [p for p in __xonsh__.subproc_captured(['ls', '-d', f'{prefix}*']).splitlines()]

completer add cdw _cdw_completer
"#,
        _ => "# Autocomplete not supported for this shell\n",
    }.to_string()
}
