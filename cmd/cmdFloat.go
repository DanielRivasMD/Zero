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

func FloatCmd() *cobra.Command {
	d := horus.Must(domovoi.GlobalDocs())
	cmd := horus.Must(d.MakeCmd("float", runFloat,
		domovoi.WithArgs(cobra.MaximumNArgs(1)),
		domovoi.WithValidArgs(validLayouts),
	))

	cmd.PersistentFlags().StringVarP(&flagG.height, "height", "H", "100%", "pane height")
	cmd.PersistentFlags().StringVarP(&flagG.width, "width", "W", "95%", "pane width")
	cmd.PersistentFlags().StringVarP(&flagG.x, "x", "X", "10", "horizontal offset")
	cmd.PersistentFlags().StringVarP(&flagG.y, "y", "Y", "0", "vertical offset")

	return cmd
}

////////////////////////////////////////////////////////////////////////////////////////////////////

func runFloat(cmd *cobra.Command, args []string) {
	op := "zero.float"

	floatLayout := "default"
	if len(args) == 1 {
		floatLayout = args[0]
	}

	geom, err := resolveWithFlags(floatLayout)
	horus.CheckErr(
		err,
		horus.WithOp(op),
		horus.WithCategory("VALIDATION_ERROR"),
		horus.WithMessage("Failed to resolve layout geometry"),
	)

	zl := newZellijFloat(
		withName("canvas"),
		withGeometry(geom),
		withCommand("zsh"),
		withCloseOnExit(true),
	)

	horus.CheckErr(
		domovoi.ExecSh(zl.Cmd()),
		horus.WithOp(op),
		horus.WithCategory("shell_command"),
		horus.WithMessage("Failed to launch floating shell"),
	)
}

func runBat(cmd *cobra.Command, args []string) {
	op := "zero.bat"

	batArgs := []string{"--paging=always"}
	if len(args) > 0 {
		batArgs = append(batArgs, args[0])
	}

	zl := newZellijFloat(
		withName("bat"),
		withGeometry(geometryFromFlags()),
		withCommand("bat"),
		withArgs(batArgs...),
		withCloseOnExit(true),
	)

	horus.CheckErr(
		domovoi.ExecSh(zl.Cmd()),
		horus.WithOp(op),
		horus.WithCategory("shell_command"),
		horus.WithMessage("Failed to launch bat"),
	)
}

func runEditor(call string, editorOverride ...string) func(cmd *cobra.Command, args []string) {
	editor := call
	if len(editorOverride) > 0 {
		editor = editorOverride[0]
	}

	return func(cmd *cobra.Command, args []string) {
		op := "zero.editor"

		opts := []zellijOpt{
			withName(editor),
			withGeometry(geometryFromFlags()),
			withCommand(call),
			withCloseOnExit(true),
		}
		if len(args) > 0 {
			opts = append(opts, withArgs(args[0]))
		}

		zl := newZellijFloat(opts...)

		horus.CheckErr(
			domovoi.ExecSh(zl.Cmd()),
			horus.WithOp(op),
			horus.WithCategory("shell_command"),
			horus.WithMessage(fmt.Sprintf("Failed to launch %s editor", editor)),
		)
	}
}

func runEza(cmd *cobra.Command, args []string) {
	op := "zero.eza"

	ezaArgs := []string{"--header", "--long", "--icons", "--classify", "--git", "--group", "--color=always"}
	if len(args) > 0 {
		ezaArgs = append(ezaArgs, args[0])
	}

	opts := []zellijOpt{
		withName("eza"),
		withGeometry(geometryFromFlags()),
		withCommand("eza"),
		withArgs(ezaArgs...),
	}

	zl := newZellijFloat(opts...)

	horus.CheckErr(
		domovoi.ExecSh(zl.Cmd()),
		horus.WithOp(op),
		horus.WithCategory("shell_command"),
		horus.WithMessage("Failed to launch eza"),
	)
}

func runLazygit(cmd *cobra.Command, args []string) {
	op := "zero.lazygit"

	floatLayout := "full"
	if len(args) == 1 {
		floatLayout = args[0]
	}

	geom, err := resolveWithFlags(floatLayout)
	horus.CheckErr(
		err,
		horus.WithOp(op),
		horus.WithCategory("VALIDATION_ERROR"),
		horus.WithMessage("Failed to resolve layout geometry"),
	)

	zl := newZellijFloat(
		withName("lazygit"),
		withGeometry(geom),
		withCommand("lazygit"),
		withCloseOnExit(true),
		withPinned(true),
	)

	horus.CheckErr(
		domovoi.ExecSh(zl.Cmd()),
		horus.WithOp(op),
		horus.WithCategory("shell_command"),
		horus.WithMessage("Failed to launch lazygit"),
	)
}

func runMDcat(cmd *cobra.Command, args []string) {
	op := "zero.mdcat"

	if len(args) < 1 {
		horus.CheckErr(
			fmt.Errorf("mdcat command requires a file argument"),
			horus.WithOp(op),
			horus.WithCategory("USAGE_ERROR"),
			horus.WithMessage("Missing file argument"),
			horus.WithExitCode(1),
		)
	}
	file := args[0]

	zl := newZellijFloat(
		withName("canvas"),
		withGeometry(geometryFromFlags()),
		withCommand("mdcat"),
		withArgs("--paginate", file),
		withCloseOnExit(true),
	)

	horus.CheckErr(
		domovoi.ExecSh(zl.Cmd()),
		horus.WithOp(op),
		horus.WithCategory("shell_command"),
		horus.WithMessage("Failed to launch mdcat"),
	)
}

func runResize(cmd *cobra.Command, args []string) {
	op := "zero.resize"

	resizeLayout := "default"
	if len(args) == 1 {
		resizeLayout = args[0]
	}

	geom, err := resolveWithFlags(resizeLayout)
	horus.CheckErr(
		err,
		horus.WithOp(op),
		horus.WithCategory("VALIDATION_ERROR"),
		horus.WithMessage("Failed to resolve layout geometry"),
	)

	cmdResize := fmt.Sprintf(`
zellij action rename-pane float
zellij action change-floating-pane-coordinates --pane-id $ZELLIJ_PANE_ID \
--height %s \
--width %s \
--x %s \
--y %s`, geom.height, geom.width, geom.x, geom.y)

	horus.CheckErr(
		domovoi.ExecSh(cmdResize),
		horus.WithOp(op),
		horus.WithCategory("shell_command"),
		horus.WithMessage("Failed to resize floating pane"),
	)
}

func runWatch(cmd *cobra.Command, args []string) {
	op := "zero.watch"

	// TODO: check if `just watch` exist
	zl := newZellijFloat(
		withName("watch"),
		withGeometry(geometryFromFlags()),
		withCommand("just"),
		withArgs("watch"),
		withCloseOnExit(true),
		withPinned(true),
	)

	horus.CheckErr(
		domovoi.ExecSh(zl.Cmd()),
		horus.WithOp(op),
		horus.WithCategory("shell_command"),
		horus.WithMessage("Failed to launch watch command"),
	)
}

func runYazi(cmd *cobra.Command, args []string) {
	op := "zero.yazi"

	zl := newZellijFloat(
		withName("yazi"),
		withGeometry(geometryFromFlags()),
		withCommand("yazi"),
		withCloseOnExit(true),
	)

	horus.CheckErr(
		domovoi.ExecSh(zl.Cmd()),
		horus.WithOp(op),
		horus.WithCategory("shell_command"),
		horus.WithMessage("Failed to launch yazi"),
	)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
