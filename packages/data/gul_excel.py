"""
GUL Excel
Excel (XLSX) generator.

Status: ✅ Implemented
Priority: Medium
"""

import zipfile
import io
from typing import List, Any
import datetime

__version__ = "0.1.0"
__all__ = ['Workbook', 'create_xlsx']

class Workbook:
    """
    Simple XLSX Generator
    Creates a valid .xlsx file using standard libraries (zipfile, xml).
    """
    
    def __init__(self):
        self.sheets = {}
        self.active_sheet = "Sheet1"
        self.sheets[self.active_sheet] = []
        
    def add_sheet(self, title: str):
        if title not in self.sheets:
            self.sheets[title] = []
            self.active_sheet = title
            
    def write(self, row: int, col: int, value: Any, sheet: str = None):
        target_sheet = sheet or self.active_sheet
        data = self.sheets[target_sheet]
        
        # Extend rows if needed
        while len(data) <= row:
            data.append([])
            
        # Extend cols if needed
        while len(data[row]) <= col:
            data[row].append(None)
            
        data[row][col] = value
        
    def append(self, row_data: List[Any], sheet: str = None):
        target_sheet = sheet or self.active_sheet
        row_idx = len(self.sheets[target_sheet])
        for col_idx, value in enumerate(row_data):
            self.write(row_idx, col_idx, value, target_sheet)
            
    def save(self, filename: str):
        # 1. Prepare XML contents
        content_types = self._gen_content_types()
        rels = self._gen_rels()
        workbook_xml = self._gen_workbook()
        workbook_rels = self._gen_workbook_rels()
        
        # 2. Write Zip
        with zipfile.ZipFile(filename, 'w', zipfile.ZIP_DEFLATED) as z:
            z.writestr('[Content_Types].xml', content_types)
            z.writestr('_rels/.rels', rels)
            z.writestr('xl/workbook.xml', workbook_xml)
            z.writestr('xl/_rels/workbook.xml.rels', workbook_rels)
            
            # Write sheets
            for i, (name, data) in enumerate(self.sheets.items()):
                sheet_xml = self._gen_sheet(data)
                z.writestr(f'xl/worksheets/sheet{i+1}.xml', sheet_xml)

    # XML Generators
    
    def _gen_content_types(self):
        sheets = "".join([f'<Override PartName="/xl/worksheets/sheet{i+1}.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>' for i in range(len(self.sheets))])
        return f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
{sheets}
</Types>"""

    def _gen_rels(self):
        return """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>
</Relationships>"""

    def _gen_workbook(self):
        sheets = "".join([f'<sheet name="{name}" sheetId="{i+1}" r:id="rId{i+1}"/>' for i, name in enumerate(self.sheets.keys())])
        return f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<sheets>{sheets}</sheets>
</workbook>"""

    def _gen_workbook_rels(self):
        rels = "".join([f'<Relationship Id="rId{i+1}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet{i+1}.xml"/>' for i in range(len(self.sheets))])
        return f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
{rels}
</Relationships>"""

    def _gen_sheet(self, data):
        rows = []
        for r_idx, row in enumerate(data):
            cols = []
            for c_idx, val in enumerate(row):
                if val is None: continue
                
                # Basic type handling
                t = "str"
                v = str(val)
                if isinstance(val, (int, float)):
                    t = "n"
                
                # Excel cell ref (A1, B1, etc.) - Simplified 1-based indexing
                # Proper conversion needed for columns > Z
                col_letter = chr(65 + c_idx) if c_idx < 26 else "A" # Hacky limit
                ref = f"{col_letter}{r_idx+1}"
                
                if t == "str":
                     cols.append(f'<c r="{ref}" t="inlineStr"><is><t>{v}</t></is></c>')
                else:
                     cols.append(f'<c r="{ref}" t="n"><v>{v}</v></c>')
            
            rows.append(f'<row r="{r_idx+1}">{"".join(cols)}</row>')
            
        return f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
<sheetData>{"".join(rows)}</sheetData>
</worksheet>"""

def create_xlsx(data: List[List[Any]], filename: str):
    wb = Workbook()
    for row in data:
        wb.append(row)
    wb.save(filename)
