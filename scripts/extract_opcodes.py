from bs4 import BeautifulSoup
import pandas as pd
import re

# Load the HTML file
html_file = "6502_opcode_reference.html"
with open(html_file, "r", encoding="windows-1252") as file:
    soup = BeautifulSoup(file, "html.parser")

# Find all instruction sections
instructions = soup.find_all("a", attrs={"name": True})

headers = list()
for header in soup.find_all("h3"):
    headers.append(header.contents[1])

data = []

for instr in instructions:
    instr_name = instr["name"].strip()

    description = ""
    para = instr.find_next("p")
    while para and not str(para.contents[0]).startswith("Processor Status after use:") and not str(para.contents[0]).startswith("<table"):
        description += f"\n\t{para.contents[0].replace("\n", " ")}"
        para = para.find_next("p")

    header = ""
    for desc in headers:
        if desc.startswith(instr_name):
            header = desc
            break

    print("/*")
    print(f"\t{header}")
    print(description)
    print("*/")

    table = instr.find_next("table").find_next("table")
    
    if not table:
        print(f"No table found for instruction {instr_name}")
        continue

    rows = table.find_all("tr")[1:]  # Skip header row

    for row in rows:
        cols = row.find_all("td")
        if len(cols) >= 4:

            p = re.compile(r'(\(|\)|,|\s+)')
            addr_mode = re.sub(p, '', cols[0].text)
            opcode = cols[1].text.strip()
            bytes_used = cols[2].text.strip()
            cycles = cols[3].text.strip()

            comment = ""
            if len(cycles) > 1:
                comment = f"// {cycles[1:].replace('\n', ' ').replace("(", "").replace(")", "")}"
                cycles = cycles[0]
                comment = ' '.join(comment.split())
                comment = comment.replace("+1 if branch", "+1 cycles if branch")

            opcode = re.sub(r'\$', "0x", opcode)

            print(f"opcode_entry!(map,     {opcode},      {instr_name},      {bytes_used},      {cycles},       {addr_mode : <12}); {comment}")

    # Add a space between each section of opcodes
    print("")
