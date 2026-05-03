/*
Copyright © 2026 Daniel Rivas <danielrivasmd@gmail.com>

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

const MONITOR = "monitor"

////////////////////////////////////////////////////////////////////////////////////////////////////

func MonitorCmd() *cobra.Command {
	d := horus.Must(domovoi.GlobalDocs())
	cmd := horus.Must(d.MakeCmd("monitor", runMonitor,
		domovoi.WithArgs(cobra.MinimumNArgs(0)),
	))

	return cmd
}

////////////////////////////////////////////////////////////////////////////////////////////////////

func runMonitor(cmd *cobra.Command, args []string) {
	const op = "zero.monitor"

	cmdStr := fmt.Sprintf(
		`zellij action new-tab --layout $HOME/.zero/layouts/%s.kdl --name $( [ "$PWD" = "$HOME" ] && echo "~" || basename "$PWD" )`,
		MONITOR,
	)

	if err := domovoi.ExecSh(cmdStr); err != nil {
		horus.CheckErr(
			err,
			horus.WithOp(op),
			horus.WithCategory("ZELLIJ_ERROR"),
			horus.WithMessage("failed to launch tab"),
		)
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
