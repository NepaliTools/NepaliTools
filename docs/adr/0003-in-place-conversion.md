# The in-place shortcut reads the selection through the clipboard

Status: accepted

Windows gives an application no supported way to read or replace the current
selection in another process. UI Automation exposes text in some controls and
nothing in many others, and it cannot write back reliably. The shortcut therefore
does what a user would do: it sends Ctrl+C, reads the clipboard, converts the
text, writes the result, sends Ctrl+V, and restores what the clipboard held
before.

## Consequences

- The clipboard is briefly replaced. Only the text part is preserved; an image or
  rich content on the clipboard is lost when the shortcut runs.
- Keystrokes are injected into whatever window has focus. A window that refuses
  synthetic input is a documented limit, not a bug; elevated windows and some
  games reject it.
- The chord fires while Ctrl and Alt are still physically held, so the app waits
  for the modifiers to come up before injecting its own Ctrl+C. Without that
  wait the target sees Alt+C and copies nothing.
- If the selection is empty, the copy produces no clipboard change. That is
  indistinguishable from a refused copy, so it is reported as "no selection"
  rather than converting stale clipboard content.
- The shortcut refuses to run when the panel has focus, because injecting keys
  into our own webview would paste into the converter box.
- Two shortcuts exist because a direction cannot be detected in every case:
  `Ctrl+Alt+P` (Unicode to Preeti) and `Ctrl+Alt+U` (Preeti to Unicode). In auto
  mode only `Ctrl+Alt+P` is registered and the direction follows the script of
  the selection.

A native implementation per target application is not worth the scope: the
clipboard path works in Notepad, Chromium and Electron applications, which is the
range this tool serves.
