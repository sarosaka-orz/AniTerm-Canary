#!/bin/bash

# AniTerm Shell Hook Setup Script
# This script detects the current shell and provides the necessary configuration.

detect_shell() {
    # 優先檢查父進程名稱
    local parent_cmd=$(ps -p $PPID -o comm= 2>/dev/null | sed 's/^-//')
    case "$parent_cmd" in
        zsh*)  echo "zsh"; return ;;
        bash*) echo "bash"; return ;;
        fish*) echo "fish"; return ;;
    esac

    # 次之檢查 $SHELL 環境變數
    case "$SHELL" in
        *zsh*)  echo "zsh"; return ;;
        *bash*) echo "bash"; return ;;
        *fish*) echo "fish"; return ;;
    esac

    # 最後檢查內部變數 (但因為腳本在 bash 中執行，這通常會回傳 bash)
    if [ -n "$ZSH_VERSION" ]; then
        echo "zsh"
    elif [ -n "$BASH_VERSION" ]; then
        echo "bash"
    else
        echo "unknown"
    fi
}

SHELL_TYPE=$(detect_shell)

# Find the aniterm binary directory
ANITERM_BIN_DIR=""
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ -f "$SCRIPT_DIR/../target/release/aniterm" ]; then
    ANITERM_BIN_DIR="$SCRIPT_DIR/../target/release"
elif [ -f "./target/release/aniterm" ]; then
    ANITERM_BIN_DIR="$(pwd)/target/release"
fi

if [ "$SHELL_TYPE" == "zsh" ]; then
    if [ -n "$ANITERM_BIN_DIR" ]; then
        echo "export PATH=\"$ANITERM_BIN_DIR:\$PATH\""
        # 提示使用者建立設定檔目錄
        mkdir -p ~/.config/aniterm 2>/dev/null
        if [ ! -f ~/.config/aniterm/config.toml ] && [ -f "$ANITERM_BIN_DIR/../../config.toml" ]; then
            cp "$ANITERM_BIN_DIR/../../config.toml" ~/.config/aniterm/config.toml 2>/dev/null
        fi
    fi
    cat << EOF
# AniTerm Zsh Hook
aniterm_preexec() {
    export ANITERM_LAST_CMD="\$1"
}

aniterm_precmd() {
    if [ -n "\$ANITERM_LAST_CMD" ]; then
        case "\$ANITERM_LAST_CMD" in
            aniterm*) ;; # 跳過 aniterm 相關指令以免造成雙氣泡
            *) aniterm --hook -- "\$ANITERM_LAST_CMD" ;;
        esac
        unset ANITERM_LAST_CMD
    fi
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec aniterm_preexec
add-zsh-hook precmd aniterm_precmd
EOF
elif [ "$SHELL_TYPE" == "bash" ]; then
    if [ -n "$ANITERM_BIN_DIR" ]; then
        echo "export PATH=\"$ANITERM_BIN_DIR:\$PATH\""
        mkdir -p ~/.config/aniterm 2>/dev/null
        if [ ! -f ~/.config/aniterm/config.toml ] && [ -f "$ANITERM_BIN_DIR/../../config.toml" ]; then
            cp "$ANITERM_BIN_DIR/../../config.toml" ~/.config/aniterm/config.toml 2>/dev/null
        fi
    fi
    cat << EOF
# AniTerm Bash Hook
aniterm_bash_hook() {
    local exit_code=\$?
    local last_cmd=\$(history 1 | sed 's/^[ ]*[0-9]*[ ]*//')
    if [ -n "\$last_cmd" ] && [ "\$last_cmd" != "\$ANITERM_PREV_CMD" ]; then
        case "\$last_cmd" in
            aniterm*) ;;
            *) aniterm --hook -- "\$last_cmd" ;;
        esac
        export ANITERM_PREV_CMD="\$last_cmd"
    fi
}
PROMPT_COMMAND="aniterm_bash_hook; \$PROMPT_COMMAND"
EOF
elif [ "$SHELL_TYPE" == "fish" ]; then
    if [ -n "$ANITERM_BIN_DIR" ]; then
        echo "set -gx PATH \"$ANITERM_BIN_DIR\" \$PATH"
        echo "mkdir -p ~/.config/aniterm"
        if [ -f "$ANITERM_BIN_DIR/../../config.toml" ]; then
            echo "if not test -f ~/.config/aniterm/config.toml; cp \"$ANITERM_BIN_DIR/../../config.toml\" ~/.config/aniterm/config.toml; end"
        fi
    fi
    cat << EOF
# AniTerm Fish Hook
function aniterm_postexec --on-event fish_postexec
    # \$argv[1] contains the command string
    if not string match -q "aniterm*" "\$argv[1]"
        aniterm --hook -- "\$argv[1]"
    end
end
EOF
else
    echo "# Unknown shell. Manual setup required."
fi
