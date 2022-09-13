from dataclasses import dataclass
from datetime import datetime
from typing import List
from xmlrpc.client import DateTime
import spacy
from bs4 import BeautifulSoup

nlp = spacy.load('en_core_web_trf')

@dataclass
class Event:
  date: datetime
  description: str

def extract(html: str) -> List[Event]:
  parsed = BeautifulSoup(html, 'html.parser')
  for child in parsed.children:
    pass
  
def _get_dates(text: str) -> List[DateTime]:
  doc = nlp(text)
  if doc.ents: 
    for ent in doc.ents:
      print(ent.text + ' - ' +str(ent.start_char) +' - '+ str(ent.end_char) +' - '+ent.label_+ ' - '+ str(spacy.explain(ent.label_)))
  else:
    print('No named entities found.')