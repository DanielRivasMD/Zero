/*
Copyright © 2025 Daniel Rivas <danielrivasmd@gmail.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
*/
package cmd

////////////////////////////////////////////////////////////////////////////////////////////////////

import (
	"fmt"

	"github.com/DanielRivasMD/domovoi"
	"github.com/DanielRivasMD/horus"
	"github.com/spf13/cobra"
)

////////////////////////////////////////////////////////////////////////////////////////////////////

var (
	launchFlags struct {
		layout string
		target string
	}
)

////////////////////////////////////////////////////////////////////////////////////////////////////

func LaunchCmd() *cobra.Command {
	d := horus.Must(domovoi.GlobalDocs())
	cmd := horus.Must(d.MakeCmd("launch", runLaunch))

	cmd.Flags().StringVarP(&launchFlags.layout, "layout", "l", "", "the .kdl layout file to launch (required)")
	cmd.Flags().StringVarP(&launchFlags.target, "target", "t", "", "if set, cd into this path before launching (and return afterward)")

	return cmd
}

////////////////////////////////////////////////////////////////////////////////////////////////////

func runLaunch(cmd *cobra.Command, args []string) {
	op := "zero.launch"

	if launchFlags.layout == "" {
		horus.CheckErr(
			fmt.Errorf("--layout flag is required"),
			horus.WithOp(op),
			horus.WithCategory("USAGE_ERROR"),
			horus.WithMessage("Missing required flag --layout"),
			horus.WithExitCode(1),
		)
	}

	cmdLaunch := fmt.Sprintf(
		`zellij action write-chars "zellij --new-session-with-layout $HOME/.config/zellij/layouts/%s"; zellij action write 13`,
		launchFlags.layout,
	)

	if launchFlags.target == "" {
		horus.CheckErr(
			domovoi.ExecSh(cmdLaunch),
			horus.WithOp(op),
			horus.WithCategory("shell_command"),
			horus.WithMessage("Failed to launch new Zellij session"),
		)
		return
	}

	const cmdZellijTab = `zellij action new-tab \
--layout $HOME/.zero/layouts/launch.kdl \
--name "$( [ "$PWD" = "$HOME" ] && echo "~" || basename "$PWD" )"`
	fullCmd := cmdZellijTab + "; " + cmdLaunch

	originalDir, err := domovoi.RecallDir()
	horus.CheckErr(err,
		horus.WithOp(op),
		horus.WithCategory("DIR_ERROR"),
		horus.WithMessage("Failed to recall original directory"),
	)

	defer func() {
		if err := domovoi.ChangeDir(originalDir); err != nil {
			horus.CheckErr(err,
				horus.WithOp(op),
				horus.WithCategory("DIR_ERROR"),
				horus.WithMessage("Failed to revert to original directory"),
				horus.WithExitCode(0),
			)
		}
	}()

	if err := domovoi.ChangeDir(launchFlags.target); err != nil {
		horus.CheckErr(err,
			horus.WithOp(op),
			horus.WithCategory("DIR_ERROR"),
			horus.WithMessage(fmt.Sprintf("Failed to change to target directory %q", launchFlags.target)),
		)
	}

	horus.CheckErr(
		domovoi.ExecSh(fullCmd),
		horus.WithOp(op),
		horus.WithCategory("shell_command"),
		horus.WithMessage("Failed to launch new Zellij session with target"),
	)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
