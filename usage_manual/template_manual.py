from jinja2 import Template
import subprocess
import os

# Configuration: Path to the binary built by cargo-dist
BINARY_PATH = "target/debug/wt_ext_cli"

# Ensure the binary exists and is executable
if not os.path.isfile(BINARY_PATH):
    raise FileNotFoundError(f"Binary not found at {BINARY_PATH}")
if not os.access(BINARY_PATH, os.X_OK):
    raise PermissionError(f"Binary at {BINARY_PATH} is not executable")

# Generate dynamic content by running the binary
try:
    top_level_help = subprocess.run(
        [BINARY_PATH, "--help"], capture_output=True, text=True, check=True
    ).stdout.strip()
except subprocess.CalledProcessError as e:
    raise RuntimeError(f"Error running binary: {e}")

# Example: Add version info (optional)
try:
    vromf_help = subprocess.run(
        [BINARY_PATH, "unpack_vromf", "--help"], capture_output=True, text=True, check=True
    ).stdout.strip()
except subprocess.CalledProcessError as e:
    raise RuntimeError(f"Error fetching version info: {e}")

try:
    blk_help = subprocess.run(
        [BINARY_PATH, "unpack_raw_blk", "--help"], capture_output=True, text=True, check=True
    ).stdout.strip()
except subprocess.CalledProcessError as e:
    raise RuntimeError(f"Error fetching version info: {e}")

try:
    vromf_version = subprocess.run(
        [BINARY_PATH, "vromf_version", "--help"], capture_output=True, text=True, check=True
    ).stdout.strip()
except subprocess.CalledProcessError as e:
    raise RuntimeError(f"Error fetching version info: {e}")

# Read the Markdown template
with open("./usage_manual/template_manual.md", "r") as file:
    template_content = file.read()

# Render the Markdown file
template = Template(template_content)
rendered_content = template.render(
    TOP_LEVEL_HELP=top_level_help,
    VROMF_HELP=vromf_help,
    BLK_HELP=blk_help,
    VROMF_VERSION=vromf_version,
)
print(rendered_content)

# Write the rendered content to the manual
output_file = "MANUAL.md"
with open(output_file, "w") as file:
    file.write(rendered_content)

print(f"Manual has been generated and written to {output_file}")
