wayshot(1) "github.com/waycrate/wayshot" "General Commands Manual"

# NAME

Wayshot - Screenshot tool for compositors implementing zwlr_screencopy_v1 such as sway and river

# SYNOPSIS

*wayshot* [_options_]

# OPTIONS

*-h*, *--help*
	Print help message and quit.

*-V*, *--version*
	Print version information.

*-d*, *--debug*
	Enable debug mode.

*-c*, *--cursor*
	Enable cursor visibility in screenshots.

*-e*, *--extension*
	Set the image encoder.
	Valid arguments:
		- jpeg
		- jpg
		- png (Default encoder)
		- ppm

*-f*, *--file*
	Set a custom file path. The default path is `./{current_unix_timestamp}-wayshot.{encoder}`
	eg: 1659034753-wayshot.png

*-l*, *--listoutputs*
	List all valid output names. This flag is generally used in combination with *-o* flag.

*-o*, *--output*
	Choose a particular display (wl_output) to screenshot.

*-s*, *--slurp*
	Choose a portion of your display to screenshot using the slurp program.
	https://github.com/emersion/slurp

*--stdout*
	Emit image data to stdout. The following flag is helpful to pipe image data
	to other programs.

# KNOWN BUGS

Feel free to send patches for the following:
- *--slurp* flag does not work as intended on multi monitor systems. After multiple attempts at fixing this I have failed time and again.

# SEE ALSO
	- wayshot(7)

# AUTHORS

Maintained by Shinyzenith <aakashsensharma@gmail.com>.
For more information about development, see <https://github.com/waycrate/wayshot>.
