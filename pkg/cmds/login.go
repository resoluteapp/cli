package cmds

import (
	"fmt"

	"github.com/spf13/cobra"
)

var loginCMD = &cobra.Command{
	Use:   "login",
	Short: "Login to resolute",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("foo bar")
	},
}

func init() {
	RootCMD.AddCommand(loginCMD)
}
