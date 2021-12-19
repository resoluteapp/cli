package cmds

import (
	"fmt"

	"github.com/resoluteapp/cli/pkg/out"
	"github.com/spf13/cobra"
)

var RootCMD = &cobra.Command{
	Use:   "resolute",
	Short: "CLI (Command Line Interface) for Resolute",
	Long: fmt.Sprintf(`
%v
`, out.Logo()),
}
