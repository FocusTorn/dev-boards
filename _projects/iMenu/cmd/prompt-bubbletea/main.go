package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

// Text input model
type inputModel struct {
	textInput textinput.Model
	message   string
	done      bool
}

func initialInputModel(message, defaultValue string) inputModel {
	ti := textinput.New()
	ti.Placeholder = ""
	ti.SetValue(defaultValue)
	ti.Focus()
	ti.CharLimit = 0
	ti.Width = 50

	return inputModel{
		textInput: ti,
		message:   message,
		done:      false,
	}
}

func (m inputModel) Init() tea.Cmd {
	return textinput.Blink
}

func (m inputModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "esc":
			return m, tea.Quit
		case "enter":
			m.done = true
			return m, tea.Quit
		}
	}

	var cmd tea.Cmd
	m.textInput, cmd = m.textInput.Update(msg)
	return m, cmd
}

func (m inputModel) View() string {
	if m.done {
		return ""
	}
	return fmt.Sprintf("%s\n\n%s\n\n(Enter to confirm, Esc to cancel)",
		m.message,
		m.textInput.View(),
	)
}

// Select model
type selectModel struct {
	cursor   int
	choices  []string
	message  string
	selected string
	done     bool
}

func initialSelectModel(message string, choices []string) selectModel {
	return selectModel{
		cursor:  0,
		choices: choices,
		message: message,
		done:    false,
	}
}

func (m selectModel) Init() tea.Cmd {
	return nil
}

func (m selectModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "esc", "q":
			return m, tea.Quit
		case "up", "k":
			if m.cursor > 0 {
				m.cursor--
			}
		case "down", "j":
			if m.cursor < len(m.choices)-1 {
				m.cursor++
			}
		case "enter":
			m.selected = m.choices[m.cursor]
			m.done = true
			return m, tea.Quit
		}
	}
	return m, nil
}

func (m selectModel) View() string {
	if m.done {
		return ""
	}

	var s strings.Builder
	s.WriteString(m.message)
	s.WriteString("\n\n")

	for i, choice := range m.choices {
		cursor := " "
		if m.cursor == i {
			cursor = ">"
			style := lipgloss.NewStyle().Foreground(lipgloss.Color("205"))
			s.WriteString(fmt.Sprintf("%s %s\n", cursor, style.Render(choice)))
		} else {
			s.WriteString(fmt.Sprintf("%s %s\n", cursor, choice))
		}
	}

	s.WriteString("\n(↑↓ to navigate, Enter to select, Esc to cancel)")
	return s.String()
}

// Confirm model
type confirmModel struct {
	message string
	yes     bool
	done    bool
}

func initialConfirmModel(message string) confirmModel {
	return confirmModel{
		message: message,
		yes:     false,
		done:    false,
	}
}

func (m confirmModel) Init() tea.Cmd {
	return nil
}

func (m confirmModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "esc", "n", "N":
			m.yes = false
			m.done = true
			return m, tea.Quit
		case "y", "Y", "enter":
			m.yes = true
			m.done = true
			return m, tea.Quit
		case "left", "h":
			m.yes = false
		case "right", "l":
			m.yes = true
		}
	}
	return m, nil
}

func (m confirmModel) View() string {
	if m.done {
		return ""
	}

	yesStyle := lipgloss.NewStyle().Padding(0, 1)
	noStyle := lipgloss.NewStyle().Padding(0, 1)

	if m.yes {
		yesStyle = yesStyle.Background(lipgloss.Color("205")).Foreground(lipgloss.Color("230"))
		noStyle = noStyle.Foreground(lipgloss.Color("240"))
	} else {
		yesStyle = yesStyle.Foreground(lipgloss.Color("240"))
		noStyle = noStyle.Background(lipgloss.Color("205")).Foreground(lipgloss.Color("230"))
	}

	return fmt.Sprintf("%s\n\n[%s] [%s]\n\n(Y/N, ←→ to switch, Enter to confirm, Esc to cancel)",
		m.message,
		yesStyle.Render("Yes"),
		noStyle.Render("No"),
	)
}

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "Usage: %s <type> [options]\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "Types: input, select, confirm\n")
		os.Exit(1)
	}

	promptType := os.Args[1]
	var p *tea.Program
	var result string

	switch promptType {
	case "input":
		message := "Enter value:"
		defaultValue := ""
		if len(os.Args) > 2 {
			message = os.Args[2]
		}
		if len(os.Args) > 3 {
			defaultValue = os.Args[3]
		}
		m := initialInputModel(message, defaultValue)
		p = tea.NewProgram(m, tea.WithAltScreen())
		finalModel, err := p.Run()
		if err != nil {
			os.Exit(1)
		}
		if im, ok := finalModel.(inputModel); ok && im.done {
			result = im.textInput.Value()
		}

	case "select":
		if len(os.Args) < 3 {
			fmt.Fprintf(os.Stderr, "Usage: %s select <message> <option1> [option2] ...\n", os.Args[0])
			os.Exit(1)
		}
		message := os.Args[2]
		choices := os.Args[3:]
		if len(choices) == 0 {
			fmt.Fprintf(os.Stderr, "Error: At least one option required\n")
			os.Exit(1)
		}
		m := initialSelectModel(message, choices)
		p = tea.NewProgram(m, tea.WithAltScreen())
		finalModel, err := p.Run()
		if err != nil {
			os.Exit(1)
		}
		if sm, ok := finalModel.(selectModel); ok && sm.done {
			result = sm.selected
		} else {
			os.Exit(1)
		}

	case "confirm":
		message := "Continue?"
		if len(os.Args) > 2 {
			message = strings.Join(os.Args[2:], " ")
		}
		m := initialConfirmModel(message)
		p = tea.NewProgram(m, tea.WithAltScreen())
		finalModel, err := p.Run()
		if err != nil {
			os.Exit(1)
		}
		if cm, ok := finalModel.(confirmModel); ok && cm.done {
			if cm.yes {
				fmt.Println("yes")
				os.Exit(0)
			} else {
				fmt.Println("no")
				os.Exit(1)
			}
		} else {
			os.Exit(1)
		}

	default:
		fmt.Fprintf(os.Stderr, "Unknown prompt type: %s\n", promptType)
		os.Exit(1)
	}

	if result != "" {
		fmt.Println(result)
	}
}
