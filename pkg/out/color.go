package out

import (
	"os"
	"os/exec"
)

// Check to see if the output terminal has 256 color support
func Has256ColorSupport() bool {
	envColor := os.Getenv("TERM")
	if envColor == "xterm-256color" {
		return true
	}

	tputExecPath, err := exec.LookPath("tput")
	if err != nil {
		return false
	}

	support, err := exec.Command(tputExecPath, "colors").Output()
	if err != nil || string(support) != "256\n" {
		return false
	}
	return true
}
