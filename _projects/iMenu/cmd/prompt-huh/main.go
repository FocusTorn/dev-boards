package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/charmbracelet/huh"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "Usage: %s <type> [options] [--result-file FILE]\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "Types: input, select, confirm, multiselect\n")
		os.Exit(1)
	}

	// Parse result file option
	resultFile := ""
	args := os.Args[1:]
	for i, arg := range args {
		if arg == "--result-file" && i+1 < len(args) {
			resultFile = args[i+1]
			args = append(args[:i], args[i+2:]...)
			break
		}
	}
	
	promptType := args[0]

	// Helper to write result
	writeResult := func(data string) {
		if resultFile != "" {
			os.WriteFile(resultFile, []byte(data+"\n"), 0644)
		} else {
			fmt.Println(data)
		}
	}
	writeResultLines := func(lines []string) {
		if resultFile != "" {
			os.WriteFile(resultFile, []byte(strings.Join(lines, "\n")+"\n"), 0644)
		} else {
			for _, line := range lines {
				fmt.Println(line)
			}
		}
	}

	switch promptType {
	case "input":
		var result string
		message := "Enter value:"
		defaultValue := ""
		
		if len(args) > 1 {
			message = args[1]
		}
		if len(args) > 2 {
			defaultValue = args[2]
		}

		form := huh.NewForm(
			huh.NewGroup(
				huh.NewInput().
					Title(message).
					Description("Type your answer and press Enter").
					Value(&result).
					Placeholder(defaultValue),
			),
		)

		if err := form.Run(); err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}

		if result == "" && defaultValue != "" {
			result = defaultValue
		}
		writeResult(result)

	case "select":
		if len(args) < 2 {
			fmt.Fprintf(os.Stderr, "Usage: %s select <message> <option1> [option2] ...\n", os.Args[0])
			os.Exit(1)
		}

		var result string
		message := args[1]
		options := args[2:]

		if len(options) == 0 {
			fmt.Fprintf(os.Stderr, "Error: At least one option required\n")
			os.Exit(1)
		}

		// Build options for huh
		huhOptions := make([]huh.Option[string], len(options))
		for i, opt := range options {
			huhOptions[i] = huh.NewOption(opt, opt)
		}

		form := huh.NewForm(
			huh.NewGroup(
				huh.NewSelect[string]().
					Title(message).
					Description("Use arrow keys to navigate, Enter to select").
					Options(huhOptions...).
					Value(&result),
			),
		)

		if err := form.Run(); err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}

		writeResult(result)

	case "confirm":
		var result bool
		message := "Continue?"
		if len(args) > 1 {
			message = strings.Join(args[1:], " ")
		}

		form := huh.NewForm(
			huh.NewGroup(
				huh.NewConfirm().
					Title(message).
					Description("Use arrow keys to switch, Enter to confirm").
					Value(&result),
			),
		)

		if err := form.Run(); err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}

		if result {
			writeResult("yes")
			os.Exit(0)
		} else {
			writeResult("no")
			os.Exit(1)
		}

	case "multiselect":
		if len(args) < 2 {
			fmt.Fprintf(os.Stderr, "Usage: %s multiselect <message> <option1> [option2] ...\n", os.Args[0])
			os.Exit(1)
		}

		var result []string
		message := args[1]
		options := args[2:]

		if len(options) == 0 {
			fmt.Fprintf(os.Stderr, "Error: At least one option required\n")
			os.Exit(1)
		}

		// Build options for huh
		huhOptions := make([]huh.Option[string], len(options))
		for i, opt := range options {
			huhOptions[i] = huh.NewOption(opt, opt)
		}

		form := huh.NewForm(
			huh.NewGroup(
				huh.NewMultiSelect[string]().
					Title(message).
					Description("Use arrow keys to navigate, Space to toggle, Enter when done").
					Options(huhOptions...).
					Value(&result),
			),
		)

		if err := form.Run(); err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}

		// Output selected options, one per line
		writeResultLines(result)

	default:
		fmt.Fprintf(os.Stderr, "Unknown prompt type: %s\n", promptType)
		os.Exit(1)
	}
}
