package out

import truecolor "github.com/wayneashleyberry/truecolor/pkg/color"

// Output the Resolute logo in ascii
func Logo() string {
	logo := `                       _       _
   _ __ ___  ___  ___ | |_   _| |_ ___
  | '__/ _ \/ __|/ _ \| | | | | __/ _ \
  | | |  __/\__ \ (_) | | |_| | ||  __/
  |_|  \___||___/\___/|_|\__,_|\__\___|
  --------------------------------------`
	if Has256ColorSupport() {
		return truecolor.Color(7, 150, 105).Sprint(logo)
	}
	return logo
}
