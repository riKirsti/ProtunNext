"""Exercise the real locked CLI on harmless files without creating a proposal."""

import json
import os
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory

import opencode_ci


def completed_tools(output):
    tools = set()
    for line in output.splitlines():
        if not line.strip():
            continue
        event = json.loads(line)
        if event["type"] != "tool_use":
            continue
        part = event["part"]
        state = part["state"]
        if state["status"] != "completed":
            continue
        tools.add(part["tool"])
        if part["tool"] == "execute":
            tools.update(call["tool"].rsplit(".", 1)[-1]
                         for call in state.get("metadata", {}).get("toolCalls", [])
                         if call["status"] == "completed")
    return tools


def main():
    key = os.environ.pop("OPENCODE_API_KEY", "")
    if not key:
        raise SystemExit("OpenCode smoke requires the repository inference key.")
    with TemporaryDirectory(prefix="opencode-smoke-") as folder:
        root = Path(folder)
        project = root / "project"
        project.mkdir()
        (project / "allowed.txt").write_text("before\n", encoding="utf-8")
        (project / "protected.txt").write_text("preserve\n", encoding="utf-8")
        (root / "outside.txt").write_text("outside\n", encoding="utf-8")
        output = opencode_ci.run(
            key, "Use the web search tool to find the official Rust compiler documentation. "
            "Use the URL fetch tool to open https://example.com/. "
            "Use the shell tool to run printf 'verified\\n' > allowed.txt. "
            "Use Code Mode if that is how the tools are exposed. Keep other files intact.",
            project,
        )
        tools = completed_tools(output)
        missing = {"websearch", "shell"} - tools
        if not tools & {"webfetch", "fetch"}:
            missing.add("webfetch")
        if missing:
            raise ValueError(f"Missing successful tool calls: {', '.join(sorted(missing))}")
        if ((project / "allowed.txt").read_text(encoding="utf-8") != "verified\n"
                or (project / "protected.txt").read_text(encoding="utf-8") != "preserve\n"
                or (root / "outside.txt").read_text(encoding="utf-8") != "outside\n"
                or {path.name for path in project.iterdir()} != {"allowed.txt", "protected.txt"}):
            raise ValueError("The synthetic edit was missing or outside its scope")
        print(f"OpenCode verified search, URL fetch and shell with {opencode_ci.MODEL}.")


if __name__ == "__main__":
    try:
        main()
    except ValueError as error:
        if type(error) is ValueError:
            print(str(error), flush=True)
        raise SystemExit("OpenCode smoke failed. No proposal or release was created.") from None
    except (OSError, TypeError, KeyError, subprocess.CalledProcessError) as error:
        print(f"OpenCode smoke error category: {type(error).__name__}", flush=True)
        raise SystemExit("OpenCode smoke failed. No proposal or release was created.") from None
