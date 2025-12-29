"""
GUL Docx
Word (DOCX) generator.

Status: ✅ Implemented
Priority: Medium
"""

import zipfile
from typing import List

__version__ = "0.1.0"
__all__ = ['Document', 'create_docx']

class Document:
    """
    Simple DOCX Generator (Pure Python)
    """
    
    def __init__(self):
        self.body_elements = []
        
    def add_paragraph(self, text: str, style: str = None):
        self.body_elements.append(f'<w:p><w:r><w:t>{text}</w:t></w:r></w:p>')
        
    def add_heading(self, text: str, level: int = 1):
        # Very simplified heading style mapping
        self.body_elements.append(f'<w:p><w:pPr><w:pStyle w:val="Heading{level}"/></w:pPr><w:r><w:t>{text}</w:t></w:r></w:p>')
        
    def save(self, filename: str):
        content_types = self._gen_content_types()
        rels = self._gen_rels()
        document_xml = self._gen_document()
        
        with zipfile.ZipFile(filename, 'w', zipfile.ZIP_DEFLATED) as z:
            z.writestr('[Content_Types].xml', content_types)
            z.writestr('_rels/.rels', rels)
            z.writestr('word/document.xml', document_xml)

    def _gen_content_types(self):
        return """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"""

    def _gen_rels(self):
        return """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"""

    def _gen_document(self):
        body = "".join(self.body_elements)
        return f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:body>{body}</w:body>
</w:document>"""

def create_docx(text: str, filename: str):
    doc = Document()
    for line in text.split('\n'):
        if line.startswith('# '):
            doc.add_heading(line[2:], 1)
        else:
            doc.add_paragraph(line)
    doc.save(filename)
