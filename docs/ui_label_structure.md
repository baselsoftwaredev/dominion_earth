# Label UI Structure

## Visual Layout

### Unit Label (120x80 pixels)

```
┌─────────────────────────────────┐
│  ┌──────────┐                   │
│  │          │  Spearman         │
│  │ Icon     │  (Rome)           │
│  │ (40x40)  │                   │
│  └──────────┘                   │
│                                 │
│  Background: Darkened Civ Color │
│  Icon: Full Brightness Civ Color│
│  Text: White                    │
└─────────────────────────────────┘
```

### Capital Label (130x90 pixels) with Population Indicator

```
┌──────────────────────────────────┐
│  ┌──────────┐                    │
│  │          │  Rome              │
│  │ Icon     │  (Civilization)    │
│  │ (45x45)  │                    │
│  └──────────┘                    │
│                                  │
│  Background: Darkened Civ Color  │
│  Icon: Full Brightness Civ Color │
│  Text: White                     │
└──────────────────────────────────┘
          ↓
        ┌───┐
        │ 5 │  Green circle - Population (in thousands)
        └───┘
```

---

## Component Hierarchy

### Entity Tree Structure

```
CapitalLabelContainer (Transform at world position)
│   ├─ Component: LabelColors { background, icon, text }
│   ├─ Component: CapitalLabel { capital_entity, capital_position }
│   ├─ Component: DespawnOnExit(Screen::Gameplay)
│   └─ Component: DespawnOnEnter(LoadingState::Loading)
│
├─ Child 1: Background Box Sprite
│   ├─ Transform: (0, 0, z:100)
│   ├─ Sprite { color: background_color, size: 130x90 }
│   └─ GlobalTransform
│
├─ Child 2: Icon Placeholder Sprite
│   ├─ Transform: (-40, 0, z:101)  [positioned on left]
│   ├─ Sprite { color: icon_color, size: 45x45 }
│   └─ GlobalTransform
│
├─ Child 3: Text Label (Text2d)
│   ├─ Transform: (-30, 0, z:102)  [positioned on right]
│   ├─ Text2d: "Rome\n(Civilization)"
│   ├─ TextFont: { font_size: 16 }
│   ├─ TextColor: white
│   ├─ TextLayout: Justify::Left
│   └─ GlobalTransform
│
├─ Child 4: Population Indicator Circle (Sprite)
│   ├─ Transform: (0, -35, z:103)  [positioned at bottom]
│   ├─ Sprite { color: green (0,1,0), size: 20x20 }
│   └─ GlobalTransform
│
└─ Child 5: Population Text (Text2d)
    ├─ Transform: (0, -35, z:104)  [centered in circle]
    ├─ Text2d: "5"  [population / 1000]
    ├─ TextFont: { font_size: 10 }
    ├─ TextColor: white
    ├─ TextLayout: Justify::Center
    └─ GlobalTransform
```

---

## Coordinate System (Local to Container)

```
                 Y
                 ↑
                 │
        ┌────────┼────────┐
        │        │        │
    -60 │        │        │ +60
        │    ┌───┼───┐    │
        │    │   │   │    │
        │────┼───●───┼────→ X
        │    │   │   │    │
        │    └───┼───┘    │
        │        │        │
    -40 │        │        │ +40
        └────────┼────────┘
                 │
              Origin (0,0)

Box Center: (0, 0)
Icon Center: (-40, 0)  [LEFT side]
Text Start:  (-25, 0)  [RIGHT side]
```

---

## Z-Index Layering (Depth)

```
Z-Axis (depth into screen)
        ↓
    102 ─ Text Label
        │
    101 ─ Icon Sprite
        │
    100 ─ Background Box
        │
    (parent container at 100-102 range)
```

---

## Color Mapping

### Unit Label Colors

```
┌─────────────────────────────────────────────────┐
│  LabelColors Component                          │
├─────────────────────────────────────────────────┤
│                                                 │
│  background: civilization_color * 0.5 (α: 0.85)│
│              ↓                                  │
│              Used for BOX                      │
│              (Darkened for contrast)           │
│                                                 │
│  icon: civilization_color (α: 1.0)             │
│         ↓                                       │
│         Used for ICON SQUARE                   │
│         (Full brightness)                      │
│                                                 │
│  text: (1.0, 1.0, 1.0) white                   │
│         ↓                                       │
│         Used for TEXT                          │
│         (Bright white for readability)         │
│                                                 │
└─────────────────────────────────────────────────┘
```

---

## Spawning Process

```
1. spawn_unit_labels() system called
   │
   ├─→ Query for Added<MilitaryUnit> components
   │
   ├─→ For each unit found:
   │   │
   │   ├─→ Get civilization info (name, color)
   │   │
   │   ├─→ Calculate world position
   │   │   (tile_position + north_offset + pixel_offset)
   │   │
   │   ├─→ Create LabelColors from civilization_color
   │   │
   │   ├─→ Spawn LabelContainer parent entity
   │   │   ├─ Transform at world_position
   │   │   ├─ LabelColors component
   │   │   └─ UnitLabel component
   │   │
   │   ├─→ Spawn Background Box (child)
   │   │   └─ Sprite with background_color, size 120x80
   │   │
   │   ├─→ Spawn Icon Placeholder (child)
   │   │   └─ Sprite with icon_color, size 40x40
   │   │
   │   └─→ Spawn Text Label (child)
   │       └─ Text2d with unit name + civilization name
   │
   └─→ update_unit_labels() system maintains positions
       (moves container when unit moves)
```

---

## Layout Constants

```rust
// Box sizing
LABEL_BOX_WIDTH: 120.0        // pixels
LABEL_BOX_HEIGHT: 80.0        // pixels

// Icon sizing (left side)
ICON_SIZE: 40.0               // pixels (square)
ICON_PADDING: 5.0             // pixels from box edge

// Text positioning (right side)
TEXT_PADDING: 5.0             // pixels from icon

// Z-layering (depth)
BOX_Z_INDEX: 100.0
ICON_Z_INDEX: 101.0
TEXT_Z_INDEX: 102.0

// Font
UNIT_LABEL_FONT_SIZE: 14.0
CAPITAL_LABEL_FONT_SIZE: 16.0

// Positioning
NORTH_TILE_OFFSET_Y: 1        // tile units (1 tile north of unit)
CAPITAL_LABEL_VERTICAL_OFFSET_PIXELS: -40.0
```

---

## Data Flow

```
MilitaryUnit spawned
    ↓
spawn_unit_labels() detects Added<MilitaryUnit>
    ↓
Query Civilization to get color [r, g, b]
    ↓
create_label_colors() produces:
    ├─ background: [r*0.5, g*0.5, b*0.5, 0.85]
    ├─ icon: [r, g, b, 1.0]
    └─ text: [1.0, 1.0, 1.0, 1.0]
    ↓
Spawn parent container with LabelColors
    ↓
Spawn 3 children:
    ├─ Background sprite (uses background color)
    ├─ Icon sprite (uses icon color)
    └─ Text2d (uses text color)
    ↓
update_unit_labels() maintains position
    (tracks unit movement, updates container position)
```

---

## Key Features

✅ **Parent-Child Hierarchy**

- Easy repositioning: move container, all children move
- Efficient rendering: grouped transforms

✅ **Color Separation**

- Background, icon, and text colors stored separately
- Can be modified independently via `LabelColors` component

✅ **Proper Z-Ordering**

- Box behind (z: 100)
- Icon middle (z: 101)
- Text front (z: 102)

✅ **Icon-Ready**

- Placeholder sprite on left side ready for actual icons
- Replace sprite with texture asset when icons available

✅ **Scalable Layout**

- Constants control sizing and spacing
- Easy to adjust dimensions for different screen scales
