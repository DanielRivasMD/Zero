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
	"github.com/DanielRivasMD/domovoi"
	"github.com/DanielRivasMD/horus"
	"github.com/spf13/cobra"
)

////////////////////////////////////////////////////////////////////////////////////////////////////

func KillCmd() *cobra.Command {
	d := horus.Must(domovoi.GlobalDocs())
	return horus.Must(d.MakeCmd("kill", runKill))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

func runKill(cmd *cobra.Command, args []string) {
	op := "zero.kill"
	cmdKill := `zellij kill-session "$(zellij list-sessions | grep '(current)' | sed 's/\x1b\[[0-9;]*m//g' | awk '{print $1}')"`
	horus.CheckErr(
		domovoi.ExecSh(cmdKill),
		horus.WithOp(op),
		horus.WithCategory("shell_command"),
		horus.WithMessage("Failed to kill current Zellij session"),
	)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
