# Step 10: AMS Custom Filament Management
**Goal:** Make your custom brand (Overture) appear in the A1's internal dropdowns.

Since you never use the physical screen, you can use Orca Slicer to "push" custom brand names into the printer's reserved memory slots. This allows the printer to "know" it has Overture ASA loaded, rather than just "Generic ASA."

## 1. Enable Custom Filaments
1.  Open **Orca Slicer**.
2.  Go to the **Prepare** tab.
3.  Click the **Gear Icon** next to the "Filament" header.
4.  Navigate to the **Filament Management** or **Custom Filament** section.
5.  Check the box: **"Enable custom filaments"**.

## 2. Pushing "Overture" to the A1
1.  In the same menu, add a new entry:
    *   **Vendor:** `Overture`
    *   **Material Type:** `ASA`
2.  Click the **"Sync to Printer"** or **"Push to Printer"** button.
    *   *Note: This writes the brand name into one of the A1's empty firmware slots.*

## 3. Remote Mapping (Device Tab)
1.  Go to the **Device** tab in Orca Slicer.
2.  Select the AMS Slot where your ASA is loaded.
3.  In the **Brand** dropdown, you will now see **Overture**.
4.  In the **Type** dropdown, select **ASA**.
5.  Set your color.

## Why this is "Showcase" Grade
*   **Zero Conflicts:** When you hit Print, Orca Slicer sees that Slot 1 is "Overture ASA." It automatically matches it to your "Overture ASA - A1 Enclosure" profile without any manual mapping.
*   **Material Physics:** The printer uses the correct tension and feed-logic for the ASA material category while applying your custom brand name.
