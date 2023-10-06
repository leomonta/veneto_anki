pub const baseUrl: &str = "http://www.elgalepin.com/linguaveneta-vec.asp";
pub const ciave: &str = "ciave";
pub const offset: &str = "offset";
pub const wordUrl: &str = "http://www.elgalepin.com/linguaveneta-detalio.asp";
pub const ID: &str = "ID";

pub const letters: [&str; 27] = ["a", "b", "c", "d", "dh", "e", "f", "g", "gn", "h", "i", "j", "l", "m", "n", "o", "p", "r", "s", "sh", "t", "u", "v", "x", "ç", "z", "zh"];

pub const wordsPerPage: u32 = 20;

pub struct Vocab {
	word: String,
	grammAbbrev: u32, // <- to enum
	useAbbrev: u32,
	entimologAbbrev: u32,
	entimologLang: u32,
	Bibliography: u32,
	placesUsed: u32, // <- prolly a vec
}

/*

Grammatical Abbreviations/Gramàdega

Note: English edition (or other printed versions) use different codes
ag. - agetivo - adjective
art. - artìcolo - article
av. - avèrbio/averbial - adverb/adverbial
cong. - congiunsion - conjunction
escl. - esclamasion/esclamadiva - exclamation
inter. - interiesion - interjection
loc. - locusion
N. - nòme - noun
inv. - invariante
inpers. - inpersonal - impersonal
fem. - feminil - feminine
ma. - maschil - masculine
pr. - pròpio - proper
prep. - prepoxision - preposition
pron. - pronòme - pronoun
vb. - vèrbo - verb
intr. - intranxitivo - intransitive
rifl. - riflesivo - reflective
tr. - tranxitivo - transitive

Use abbreviations/Uxo

arc. - arcàico - arcaic
let. - letrario - literature
ius. - de iure - juridical
pop. - popolar - popular
fam. - familiar - slang
iro. - ironìa - ironic
sch. - schersoxo - joking
feg. - feguradivo - figurative
inf. - infantil - baby talk
tri. - trivial - trivial
spr. - despresadivo - derogative
bot. - botànego - botanic
zoo. - zoolòxico - zoologic
tek. - tènico - tecnical

Etimological abbreviations/Etimoloxìa

AUT = o - or
PER = par mèdio de - through
ONOM. = onomatopèico - onomatopeic
Prob. = provàbile - probably
Ant. = antigo - ancient
Neo = novo - new

Etimological lang

Cas. = castejan - castillian
Dh. = dhermànego - germanic
Eng. = inglexe - english
Fn. = franco - franc
Fr. = francexe - french
Fur. = furlan - friulan
Gr. = grègo - greek
It. = talian - Italian
Lat. = latin - latin
Lg. = longobardo - longobard
Pers. = persian - persian
Slo. = sloven - slovenian
Tur. = turco - turkish
Ven. = venesian - venetian

Bibliographic abbreviations/Bibiografìa

A.Ven. = Anonimo del 1500 - La Venexiana
Bad.Inv. =Inventario Badoer 1521
Ben.O.F. = C. Benedetto - Orlando Furioso 1554
Boa.Ome. = F. Boaretti - Omero in Lombardia - Iliade di Omero in lingua veneta
Cal.Let. = A. Calmo - Lettere
Cas.Ome. = G. Casanova - Iliade
Chi.Zen. = Daniel da Chinazzo - Cronica de la guerra tra Veniciani e Zenovesi
Chi.Cro. = Daniel da Chinazzo - Cronica 1386
Eso. = Esopo veneto, 1449 (Cod.38023 British Library)
Fos.Pop. = Iacopo Vincenzo Foscarini - Canti del popolo veneziano, 1844
Gal.Bar. = G. Gallina - Le barufe in famegia
Gia.DeB. = Giacomin da Verona - De Babilonia civitate infernali
Gia.Jer. = Giacomin da Verona - De Jerusalem celesti
Gid.Trat.Ritmi = Gidone da Sommacampagna - Trattato e Arte deli Rithimi Volgari (Sec. XIV)
Gol.Rus. = C. Goldoni - I Rusteghi
Gol.Cam. = C. Goldoni - Il campiello
Gol.Cas. = C. Goldoni - La casa nova
Gol.Mare = C. Goldoni - La bona Mare
Gol.Bar. = C. Goldoni - Le baruffe chiozzotte
Gol.Car. = C. Goldoni - Una delle ultime sere di carnovale
Gol.Ser. = C. Goldoni - Il servitore di due padroni
Gri.Poe. = Francesco Gritti - "Poesie in dialetto veneziano"
Mar.Gal.Cioxa = Mariègola dei Galafadi de Cioxa (metà XV sec.)
Mar.Cal.Cioxa = Mariègola dei Caleghèri de Cioxa (1300-1330)
Mar.SCro.Cioxa = Mariègola de s. Croxe de Cioxa (1387)
Mor.Chro = Antonio Morosini - Chronica de Veniexia (1433)
Mua.Diz. = F. Zorzi Muazzo - Dizionario veneto, voci, frasi, sentenze, proverbi
Panfilo = Pamphilus in antico veneziano
Polo.Mil.211 = Marco Polo - Il milione veneto (CM 211 Padova)
Pro.Fem. = Anonimo Vèneto - Proverbia quae dicuntur super natura feminarum (1152-1160-1216?)
Reg.Trev. = Libro delle Regole del Territorio di Treviso - 1315
Ruz.Bil. = Ruzante - Bilora
Ruz.Red. = Ruzante - Il reduce
Ven.Str. = M. Veniero - La strazzosa
Zan.Mis. = A. Zanzotto - Mistieròi
Zib.Can. = Zibaldone da Canal (XIV sec.)

Places for phrase examples / Ljòghi de rilevamento fraxeologìa

ag = Agordin
bl = Belun
ch = Cioxa
ma = Maròstega
pd = Pàdoa
ra = Raguxa
sd = San Donà
sn = Sinistra Piave
tf = Tèra-ferma oriental
ve = Venèsia
vi = Vicensa
vr = Verona
vv = Vitòrio Vèneto
za =Zara

*/
