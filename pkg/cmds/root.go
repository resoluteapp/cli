package cmds

import (
	"github.com/resoluteapp/cli/pkg/out"
	"github.com/spf13/cobra"
)

var RootCMD = &cobra.Command{
	Use:   "resolute",
	Short: "CLI (Command Line Interface) for Resolute",
	Long:  out.Logo() + "\n  CLI (Command Line Interface) for Resolute",
	Args:  cobra.NoArgs,
}
