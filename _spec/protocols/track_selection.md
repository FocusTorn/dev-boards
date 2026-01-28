## TRACK SELECTION
**PROTOCOL: Identify and select the track to be processed.**

1.  **Check for User Input:** First, check if the user provided a track name as an argument.

2.  **Locate and Parse Tracks Registry:**
    -   Resolve the **Tracks Registry**.
    -   Read and parse this file by splitting its content by the `---` separator to identify each track section.
    -   **CRITICAL:** If no track sections are found, announce failure and halt.

3.  **Select Track:**
    -   **If a track name was provided:** Match exactly (case-insensitive) against descriptions. confirm selection: "I found track '<track_description>'. Is this correct?"
    -   **If no track name was provided:**
        1.  **Contextual Detection:** Check if the current branch matches `track/<track_id>`. If so, find that track in the registry.
        2.  **Next Available (Fallback):** Find the first track in the registry NOT marked `[x]`.
        3.  **Interactive Selection (If Ambiguous):** If multiple tracks are viable or if requested, present a numerical list of incomplete tracks and prompt the user: "Which track would you like to work on? Please enter the corresponding number."
    -   **Confirm Selection:** "I am preparing to work on track '<track_description>'. Is this correct?"
