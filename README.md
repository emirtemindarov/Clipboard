# **Clipboard**
### Inspired by *Windows Clipboard*
###### In this project I'm trying to take an idea of *Windows Clipboard* application and expand it in my own way, using `egui`.

## **Versions**
## `Release 0.1.0`
#### Non-optimized build.

#### Functionality:
- Store copied text
- Pin important blocks
- Delete one block or all not pinned at once
- Select the copied text from the block
- See text preview from block
- See more text from block
- See copy date 

#### Defects:
- Copying of Cyrillic characters is not supported and may cause a crash
- High CPU load (especially with large amounts of copied text (`.clone()` usage))
- Sometimes there is an incorrect display order of blocks

#### Roadmap:
1. Rebuild data structure
2. Minimize `.clone()` usage
3. Fix incorrect block display order
4. Add new features





#
#
#
#
#
#
#
#
#
#
#
#
#
#
#
#
###### "perfect english"