# Diablo 2 Save (.d2s) File Format Specification

This document provides a technical overview of the `.d2s` character save file format used in Diablo 2 and Diablo 2: Resurrected. It is designed to guide developers in implementing a robust parser and serializer.

## 1. Core Concepts

### Byte Order
Diablo 2 uses **Little-Endian** byte order for all multi-byte integers (`u16`, `u32`). For example, the magic number `0xAA55AA55` appears on disk as `55 AA 55 AA`.

### Bit-Packing
The `.d2s` format is highly efficient. While the header and structure prefixes use byte-alignment, the **Attributes** and **Items** sections are heavily bit-packed. A parser must support bit-level reading (MSB vs LSB) to extract variable-width fields.

---

## 2. File Structure Overview

A `.d2s` file consists of several sections, most starting with a unique 2 or 4-byte "magic number" (signature).

| Offset | Section | Size | Magic / Signature |
| :--- | :--- | :--- | :--- |
| `0x000` | [Header](#3-header) | 335 bytes | `0xAA55AA55` |
| `0x14F` | [Quest Info](#4-quest-info) | 298 bytes | `0x216F6F57` ("Wo o!") |
| `0x279` | [Waypoints](#5-waypoints) | 80 bytes | `0x5357` ("WS") |
| `0x2C9` | NPC Data | 52 bytes | N/A |
| `0x2FD` | [Attributes](#6-attributes-stats) | Variable | `0x6667` ("gf") |
| Variable | [Skills](#7-skills) | 32 bytes | `0x6669` ("if") |
| Variable | [Items](#8-items) | Variable | `0x4A4D` ("JM") |
| Variable | [Corpse](#9-additional-data) | Variable | `0x4D4A` ("JM") |
| Variable | Hireling/Merc | Variable | `0x666A` ("jf") |
| Variable | Golem | Variable | `0x666B` ("kf") |

---

## 3. Header

The header contains basic identity and status information.

> [!IMPORTANT]
> The total header size is **335 bytes**. Miscalculating this offset will cause all subsequent section signatures to fail.

- **Magic** (4 bytes): `0xAA55AA55`.
- **Version** (4 bytes): `0x60` (v1.10+), `0x61` (D2R), `0x62` (D2R 2.4+).
- **File Size** (4 bytes): Total file size in bytes.
- **Checksum** (4 bytes): CRC32 of the file data (excluding the checksum field itself).
- **Character Name** (16 bytes): Null-terminated ASCII string.
- **Character Status** (1 byte): Bitmask for Hardcore, Died, Expansion, etc.
- **Character Class** (1 byte): 0:Ama, 1:Sor, 2:Nec, 3:Pal, 4:Bar, 5:Dru, 6:Ass.
- **Level** (1 byte): 1-99.

---

## 4. Quest Info

**Signature**: `0x216F6F57` (at offset `0x14F`).

Contains completion status and rewards for all quests across three difficulties.
- **Acts** (4 bytes): Usually `6`.
- **Size** (2 bytes): Usually `298`.
- **Data** (288 bytes): 96 bytes per difficulty (Normal, Nightmare, Hell). Each difficulty tracks quests by bitfields.

---

## 5. Waypoints

**Signature**: `0x5357` (at offset `0x279`).

- **Size** (2 bytes): `80`.
- **Data**: 3 difficulty blocks. Each block typically starts with `0x0102` followed by 5 bytes of waypoint flags (each bit corresponds to a specific node).

---

## 6. Attributes (Stats)

**Signature**: `0x6667` ("gf").

This is the first **variable-length bit-stream** section.
1. Read signature `0x6667`.
2. Start reading bits:
   - Read **9 bits** for the "Stat ID".
   - If ID is `0x1FF` (511), the section ends.
   - Otherwise, look up the bit-width for that specific ID (e.g., Strength is 10 bits, Experience is 32 bits).
   - Read the specified bits for the stat value.
   - Repeat.

---

## 7. Skills

**Signature**: `0x6669` ("if").

A fixed 32-byte block:
- **Signature** (2 bytes): `0x6669`.
- **Skill Levels** (30 bytes): Each byte represents the "hard" points invested in a skill (Amazon skills first, etc.). Note that there are 30 skills per class.

---

## 8. Items

**Signature**: `0x4A4D` ("JM").

The most complex section, using **recursive parsing**.
- **Item Count** (2 bytes): Number of items in this specific list.
- **Item Data**: A continuous bit-stream.

### Item Parsing Logic:
1. Each item starts with `JM` (16 bits).
2. Read a series of bit-flags (Identified, Socketed, Ethereal, etc.).
3. Read **Item ID** (32 bits).
4. Read **Location** data (X, Y, Body Location, Container Type).
5. Detailed data (Version, Rarity, Quality, Stats) follows based on flags.
6. **Magical Attributes**: If the item has stats, they are packed similarly to Character Attributes but with different ID widths (the "Item Stat Cost" table).

---

## 9. Additional Data

- **Corpse**: Starts with `JM`. Always follows items.
- **Mercenary**: Starts with `jf`. Binary data followed by a standard Item List (`JM`).
- **Golem**: Starts with `kf`. A single byte toggle followed by a single Item (`JM`) if the golem was summoned from an item (Iron Golem).

---

## 10. Implementation Tips

### CRC32 Verification
The checksum in the header is vital. To verify:
1. Copy the save bytes.
2. Set bytes at offset `0x0C` to `0x00` (4 bytes).
3. Calculate CRC32 on the whole buffer.
4. Compare with the original value.

### Resources
- **D2MOO/D2R-Save**: Community-maintained C++ headers.
- **binrw (Rust)**: Excellent for declarative parsing of the fixed sections.
- **bitstream-io (Rust)**: Essential for the Attributes and Items sections.
