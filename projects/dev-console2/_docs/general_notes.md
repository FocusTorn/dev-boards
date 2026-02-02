
```pwsh

$i="🗑","🗑️","🗙","│📂","│📁"
foreach($d in $i) { "$d Text  |  $([char]27)[1m$d$([char]27)[22m Text" }


New-Item -ItemType SymbolicLink -Path "D:/_dev/_Projects/dev-boards/.gemini/_core/system_settings.json" -Target "C:/ProgramData/gemini-cli/settings.json"


```


/chat save api-upgrade
/chat list
/chat resume api-upgrade

/chat save settings_and_out_widgets

gemini-3-pro-preview
gemini-3-flash-preview
gemini-2.5-pro
gemini-2.5-flash
gemini-2.5-flash-lite


https://github.com/marcusgoll/Spec-Flow/blob/main/docs/getting-started.md


cargo add tui-scrollbar
https://github.com/preiter93/tui-theme-builder



---



 <!-- Start Fold: Gemini CLI custom slash commands -->

✦ The Gemini CLI supports custom slash commands through .toml files placed in a .gemini/commands/ directory. You can create a command that pulls in your specific Markdown file using the @{path} syntax.

  To set this up for your "Lessons Learned" file, follow these steps:

  1. Create the Commands Directory
  If it doesn't exist, create a .gemini/commands folder in your project root:

   1 mkdir .gemini/commands -ErrorAction SilentlyContinue

  2. Create the Command File
  Create a file named lessons.toml (or any name you prefer for the shortcut) in that directory:
   - File Path: .gemini/commands/lessons.toml

  3. Configure the Command
  Add the following content to lessons.toml. This uses the absolute path you provided to inject the content of your Markdown file:

    1 description = "Summarize lessons learned from the project"
    2
    3 # Use @{path} to inject the markdown file content
    4 # Use {{args}} if you want to pass extra info to the command (e.g. /lessons "Part 2")
    5 prompt = """
    6 Please process the following instructions and content:
    7
    8 @{D:/_dev/_Projects/dev-boards/_shared-resources/shared-cursor/commands/SumUp - 3. Lessons Learned.md}
    9
   10 Additional Context: {{args}}
   11 """

  4. Usage
  Once saved, you can trigger the command directly from the Gemini CLI:

   1 /lessons
  Or with additional input:
   1 /lessons "Focus on the networking section"

  Key Details
   - Command Name: The name is derived from the filename (e.g., lessons.toml becomes /lessons).
   - Dynamic Injection: The @{...} syntax reads the file content at execution time, so any updates to your Markdown file will be reflected immediately when you run the command.
   - Nested Commands: If you place the file in a subdirectory (e.g., .gemini/commands/summary/lessons.toml), it would be invoked as /summary:lessons.
   
   
   
   
   ---
   
<!-- Close Fold -->

 <!-- Start Fold: Orriginal Settings --> 



┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                                          ESP32-S3 Dev Console                                                        │
└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
                                                                    ╭──────────╮
┌─────────────────────────────────────────────────────── Dashboard ─╯ Settings ╰──────────────────────── [ sk6822-solo-arduino-win ] ──┐
│┌ Profiles ──────────────────┐┌ Sketch Directory ────────────────────────────────────────────────────────────────────────────────────┐│
││     sht21-Win-arduino-win  ││ D:\_dev\_Projects\dev-boards\projects\esp32-s3__LB-Gold\sk6822-solo                                  ││
││ >   sk6822-solo-arduino-wi │└──────────────────────────────────────────────────────────────────────────────────────────────────────┘│
││                            │┌ Sketch Name ─────────────────────────────────────────────────────────────────────────────────────────┐│
││                            ││ sk6822-solo                                                                                          ││
││                            │└──────────────────────────────────────────────────────────────────────────────────────────────────────┘│
││                            │┌ Device ────────────────┐┌ Connection ────────────┐┌ MQTT ──────────────────┐┌ Topics ────────────────┐│
││                            ││ ┌ Environment ───────┐ ││ ┌ Port ──────────────┐ ││ ┌ MQTT Host ─────────┐ ││ ┌ Command Topic ─────┐ ││
││                            ││ │ arduino            │ ││ │ COM9               │ ││ │                    │ ││ │                    │ ││
││                            ││ └────────────────────┘ ││ └────────────────────┘ ││ └────────────────────┘ ││ └────────────────────┘ ││
││                            ││ ┌ Board Model ───────┐ ││ ┌ Baudrate ──────────┐ ││ ┌ MQTT Port ─────────┐ ││ ┌ State Topic ───────┐ ││
││                            ││ │ esp32-s3           │ ││ │ 115200             │ ││ │                    │ ││ │                    │ ││
││                            ││ └────────────────────┘ ││ └────────────────────┘ ││ └────────────────────┘ ││ └────────────────────┘ ││
││                            ││ ┌ FQBN ──────────────┐ ││                        ││ ┌ MQTT Username ─────┐ ││ ┌ Status Topic ──────┐ ││
││                            ││ │ esp32:esp32:esp32s │ ││                        ││ │                    │ ││ │                    │ ││
││                            ││ └────────────────────┘ ││                        ││ └────────────────────┘ ││ └────────────────────┘ ││
││                            ││                        ││                        ││ ┌ MQTT Password ─────┐ ││                        ││
││                            ││                        ││                        ││ │                    │ ││                        ││
││                            ││                        ││                        ││ └────────────────────┘ ││                        ││
││                            ││                        ││                        ││                        ││                        ││
││                            ││                        ││                        ││                        ││                        ││
│└────────────────────────────┘└────────────────────────┘└────────────────────────┘└────────────────────────┘└────────────────────────┘│
└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
[q] Quit | [🡙] Navigate Profiles | [Tab] Next field | [Shift+Tab] Previous field | [Enter/Click] Confirm/Edit field
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
Status: Ready | [q] Quit








┌─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                                         ESP32-S3 Dev Console                                                        │
└─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
                                                                        ╭───────────╮
┌──────────────────────────────────────────────── Dashboard ─ Settings ─╯ Settings2 ╰──────────────────── [ sht21-Win-arduino-win ] ──┐
│  ┌ Target Sketch ────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐  │
│  │ ┌ Sketch Directory ─────────────────────────────────────────────────────────────────────────────────────────────────────────┐ │  │
│  │ │ D:\_dev\_Projects\dev-boards\projects\esp32-s3__LB-Gold\sk6822-solo                                                       │ │  │
│  │ └───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘ │  │
│  │ ┌ Sketch Name ──────────────────────────────────────────────────────────────────────────────────────────────────────────────┐ │  │
│  │ │ sk6822-solo                                                                                                               │ │  │ 
│  │ └───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘  │
│                                                                                                                                     │
│  ┌ Device ──────────────────────┐ ┌ Connection ──────────────────┐ ┌ MQTT ──────────────────────────────────────[ MQTT Profile1 ]┐  │
│  │ ┌ Environment ─────────────┐ │ │ ┌ Port ────────────────────┐ │ │ ┌ MQTT Host ────────────────┐ ┌ Command Topic ────────────┐ │  │
│  │ │ arduino                  │ │ │ │ COM9                     │ │ │ │                           │ │                           │ │  │
│  │ └──────────────────────────┘ │ │ └──────────────────────────┘ │ │ └───────────────────────────┘ └───────────────────────────┘ │  │
│  │ ┌ Board Model ─────────────┐ │ │ ┌ Baudrate ────────────────┐ │ │ ┌ MQTT Port ────────────────┐ ┌ State Topic ──────────────┐ │  │
│  │ │ esp32-s3                 │ │ │ │ 115200                   │ │ │ │                           │ │                           │ │  │ 
│  │ └──────────────────────────┘ │ │ └──────────────────────────┘ │ │ └───────────────────────────┘ └───────────────────────────┘ │  │
│  │ ┌ FQBN ────────────────────┐ │ │                              │ │ ┌ MQTT Username ────────────┐ ┌ Status Topic ─────────────┐ │  │ 
│  │ │ esp32:esp32:esp32s       │ │ │                              │ │ │                           │ │                           │ │  │
│  │ └──────────────────────────┘ │ │                              │ │ └───────────────────────────┘ └───────────────────────────┘ │  │
│  │                              │ │                              │ │ ┌ MQTT Password ────────────┐                               │  │
│  │                              │ │                              │ │ │                           │                               │  │
│  │                              │ │                              │ │ └───────────────────────────┘                               │  │
│  │                              │ │                              │ │                                                             │  │
│  │                              │ │                              │ │                                                             │  │
│  └──────────────────────────────┘ └──────────────────────────────┘ └────────────────────────────────────────[ Save ]─[ Save New ]┘  │
└─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
[q] Quit | [🡙] Navigate Profiles | [Tab] Next field | [Shift+Tab] Previous field | [Enter/Click] Confirm/Edit field
───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
Status: Ready | [q] Quit






Settings Tab

There should be no profile selector tabbar in the upper right

Input Fields should exit input mode if the mouse is clicked outside of the drop down, and a selection should be able to be made by clicking on it
All bounding box border should be rendered as white for the top and left borders, and grey for the right and bottom borders, and should not change that color unless text input or a drop down is shown





The dropdowns should be in the cyan color, not yellow




Addin an active profile section

┌───────────────────────────────────────────────────────────
│┌ Active Profile ───────────┐
││ ┌───────────────────────┐ │
││ │                       │ │
││ └───────────────────────┘ │
│└─────────[ Clear ]─[ Save ]┘
│┌ Stored Profiles ──────────┐
││   sht21-Win-arduino-win   │
││ > sk6822-solo-arduino-win │
││                           │
││                           │
││                           │
││                           │

- it will populate when a profile is made active from selecting it in the Store Profiles list
- hitting clear will remove the text from the input field
- hitting save will save the current values in the input fields as the profile name using the text in the input field 
    - If a profile already exists with the same name, prompt to overwrite it, using the confirm popup component

    
 






The "Profiles" bounding box currently is rendered like

┌─────────────────────────────────
│┌ Stored Profiles ──────────┐
││    sht21-Win-arduino-win  │
││ >  sk6822-solo-arduino-wi │
││                           │
││                           │
││                           │
││                           │



- It should be called "Stored Profiles"
- The box border should be white
- the selected line item should be a dim grey background that spans from the left border all the way accross to the right, but does not affect the border of the box itself
- The active profile is the one with the > symbol, shuld be cyan colored, and should look like the following, one space > one space and the full profile name and then one space
    │ > sk6822-solo-arduino-win │

- THe profiles box should be sized according the the width of the longest profile name and the preceeding characters
    - Assuming the above is the longest profile name it would be what ever size would show the entire name and the ` > ` and the end margin ` `
- The profile names that are listed are not affected by the modal dimming






    



Both the sketch directory and the sketch name should be enclosed in a bounding box with a title of "Target Sketch"






For the MQTT and Topics

Should look like this now


┌ MQTT ────────────────────────────────────────────────────[ MQTT Profile1 ]┐
│ ┌ MQTT Host ───────────────────────┐ ┌ Topic1 ──────────────────────────┐ │
│ │                                  │ │                                  │ │
│ └──────────────────────────────────┘ └──────────────────────────────────┘ │
│ ┌ MQTT Port ───────────────────────┐ ┌ Topic2 ──────────────────────────┐ │
│ │                                  │ │                                  │ │
│ └──────────────────────────────────┘ └──────────────────────────────────┘ │
│ ┌ MQTT Username ───────────────────┐ ┌ Topic3 ──────────────────────────┐ │
│ │                                  │ │                                  │ │
│ └──────────────────────────────────┘ └──────────────────────────────────┘ │
│ ┌ MQTT Password ───────────────────┐ ┌ Topic4 ──────────────────────────┐ │
│ │                                  │ │                                  │ │
│ └──────────────────────────────────┘ └──────────────────────────────────┘ │
│                                                                           │
└───────────────────────────────────────────────────────[ Save ]─[ Save As ]┘

a profile selectior like on the dashboard
hitting save will save the current value to the current active MQTT profile
Hitting save as will popup and modal prompt (using the component) for a name

These will be used for the MQTT monitor in the dashboard



Teh Dashyboard tab should be colored and styled like was described for the settings tab

But the commands box, leave the coloration of the selection along it is fine

the drop down needs to be corrected 
the scroll bars for the output panel do not work correctly.  The bar itself can not reach the bottom of the scrolling indicator, and it does not auto scroll correctly

The upload commands progress bar shows the completed status of the compile command ran before it.





If you see any opportinities to optimize, streamline, enhance DRY or make easier to maintain, please do so.



Any changes that are made need to have detailed documentation as to what was changed, the location of the change and the reasoning and rationale behind it.



<!-- Close Fold -->




---




ArduinoJson                                              7.4.2   D:\zLinkedFromC\Documents\Arduino\libraries\ArduinoJson
SparkFun HTU21D Humidity and Temperature Sensor Breakout 1.1.3   D:\zLinkedFromC\Documents\Arduino\libraries\SparkFun_HTU21D_Humidity_and_Temperature_Sensor_Breakout
MQTT_RPi_Client                                          1.0.0   D:\_dev\_Projects\dev-boards\lib\esp32-s3\MQTT_RPi_Client
Used platform Version Path
esp32:esp32   3.3.3   C:\Users\slett\AppData\Local\Arduino15\packages\esp32\hardware\esp32\3.3.3
Error during build: exit status 1
[Error] Command failed: Process cancelled or failed.

---


Mouse click and control or alt works, shift does not

Unless they have a vscode keybinding set for them, all three modifiers work for 
letters 
Home/End





Unless they have a vscode keybinding set for them






While your at it in the build-config.yaml these should show in bottom bar to the right side

```yaml
    # Global keyboard bindings
    bindings:
        - id: "quit"
          key: "[q]"
          display: "[q] Quit"
          onPress: "quit"
```

which works display wise, but they should be in the dispatcher available at all times, and the [??] should be cyan, and the description next to it should be a grey color

where as the 

```yaml
      # Tab-specific bindings for each tab
      tab_bindings:
      
        dashboard:
          - key: "[🡙]"
            description: "Navigate Commands"
          - key: "[🡘]"
            description: "Change Profile"
          - key: "[Enter]"
            description: "Execute Command"
```

are only available while on the dashboard tab (which we currently are) as seen below

```yaml
      tabs:
        - id: "dashboard"
          name: "Dashboard"
          default: "active"
```

but they should be in the dispatcher available when id: "dashboard" tab is active, and the [??] should be cyan, and the description next to it should be a grey color


















@https://github.com/cloudfunnels/rust-guardian                                                                                                                                                                                                     │
@https://crates.io/crates/rust-guardian                                                                                                                                                                                                            │
@https://docs.rs/rust-guardian/0.1.0/rust_guardian/ 

Based on the above documentation sites for Rust Guardian, can you write a cleanly formatted txt document (within .gemini/_docs) outlining:

- for each of the different avialale commands
  - a high level of the functionality
  - Example situations that would call for their use
  - and any other notes or detail you think is pertanent


and one for each of these gemini-cli extensions:

- rust-agentic-skills @https://github.com/udapy/rust-agentic-skills
- conductor @https://github.com/gemini-cli-extensions/conductor


