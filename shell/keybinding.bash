exec_qcl() {
    READLINE_LINE=$(qcl)
    echo "$READLINE_LINE"
}

bind -x '"\C-_": exec_qcl'
