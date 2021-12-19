package main

import (
	"log"

	"github.com/resoluteapp/cli/pkg/cmds"
)

func main() {
	err := cmds.RootCMD.Execute()
	if err != nil {
		log.Panicln(err)
	}
}
