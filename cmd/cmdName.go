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

func NameCmd() *cobra.Command {
	d := horus.Must(domovoi.GlobalDocs())
	return horus.Must(d.MakeCmd("name", runName))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

func runName(cmd *cobra.Command, args []string) {
	op := "zero.name"
	cmdRename := `zellij action rename-tab "$( [ "$PWD" = "$HOME" ] && echo "~" || basename "$PWD" )"`
	horus.CheckErr(
		domovoi.ExecSh(cmdRename),
		horus.WithOp(op),
		horus.WithCategory("shell_command"),
		horus.WithMessage("Failed to rename Zellij tab"),
	)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
