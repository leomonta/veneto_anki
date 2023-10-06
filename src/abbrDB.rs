use std::collections::HashMap;

pub fn getGrammaticalAbbreviations() -> HashMap<&'static str, &'static str> {
	let res = HashMap::from([
		("ag.", "adjective"),
		("art.", "article"),
		("av.", "adverb/adverbial"),
		("cong.", "conjunction"),
		("escl.", "exclamation"),
		("inter.", "interjection"),
		("loc.", "locution"),
		("N.", "noun"),
		("inv.", "invariant"),
		("inpers.", "impersonal"),
		("fem.", "feminine"),
		("ma.", "masculine"),
		("pr.", "proper"),
		("prep.", "preposition"),
		("pron.", "pronoun"),
		("vb.", "verb"),
		("intr.", "intransitive"),
		("rifl.", "reflective"),
		("tr.", "transitive"),
	]);

	return res;
}

pub fn getUseAbbreviations() -> HashMap<&'static str, &'static str> {
	let res = HashMap::from([
		("arc.", "arcaic"),
		("let.", "literature"),
		("ius.", "juridical"),
		("pop.", "popular"),
		("fam.", "slang"),
		("iro.", "ironic"),
		("sch.", "joking"),
		("feg.", "figurative"),
		("inf.", "baby talk"),
		("tri.", "trivial"),
		("spr.", "derogative"),
		("bot.", "botanic"),
		("zoo.", "zoologic"),
		("tek.", "tecnical"),
	]);

	return res;
}

pub fn getEtimologicalAbbreviations() -> HashMap<&'static str, &'static str> {
	let res = HashMap::from([("AUT", "or"), ("PER", "through"), ("ONOM.", "onomatopeic"), ("Prob.", "probably"), ("Ant.", "ancient"), ("Neo", "new")]);

	return res;
}

pub fn getEtimologicalLanguage() -> HashMap<&'static str, &'static str> {
	let res = HashMap::from([
		("Cas.", "castillian"),
		("Dh.", "germanic"),
		("Eng.", "english"),
		("Fn.", "franc"),
		("Fr.", "french"),
		("Fur.", "friulan"),
		("Gr.", "greek"),
		("It.", "Italian"),
		("Lat.", "latin"),
		("Lg.", "longobard"),
		("Pers.", "persian"),
		("Slo.", "slovenian"),
		("Tur.", "turkish"),
		("Ven.", "venetian"),
	]);

	return res;
}

pub fn getBibliographics() -> HashMap<&'static str, &'static str> {
	let res = HashMap::from([
		("A.Ven.", "Anonimo del 1500 - La Venexiana"),
		("Bad.Inv.", "nventario Badoer 1521"),
		("Ben.O.F.", "C. Benedetto - Orlando Furioso 1554"),
		("Boa.Ome.", "F. Boaretti - Omero in Lombardia - Iliade di Omero in lingua veneta"),
		("Cal.Let.", "A. Calmo - Lettere"),
		("Cas.Ome.", "G. Casanova - Iliade"),
		("Chi.Zen.", "Daniel da Chinazzo - Cronica de la guerra tra Veniciani e Zenovesi"),
		("Chi.Cro.", "Daniel da Chinazzo - Cronica 1386"),
		("Eso.", "Esopo veneto, 1449 (Cod.38023 British Library)"),
		("Fos.Pop.", "Iacopo Vincenzo Foscarini - Canti del popolo veneziano, 1844"),
		("Gal.Bar.", "G. Gallina - Le barufe in famegia"),
		("Gia.DeB.", "Giacomin da Verona - De Babilonia civitate infernali"),
		("Gia.Jer.", "Giacomin da Verona - De Jerusalem celesti"),
		("Gid.Trat.Ritmi", "Gidone da Sommacampagna - Trattato e Arte deli Rithimi Volgari (Sec. XIV)"),
		("Gol.Rus.", "C. Goldoni - I Rusteghi"),
		("Gol.Cam.", "C. Goldoni - Il campiello"),
		("Gol.Cas.", "C. Goldoni - La casa nova"),
		("Gol.Mare", "C. Goldoni - La bona Mare"),
		("Gol.Bar.", "C. Goldoni - Le baruffe chiozzotte"),
		("Gol.Car.", "C. Goldoni - Una delle ultime sere di carnovale"),
		("Gol.Ser.", "C. Goldoni - Il servitore di due padroni"),
		("Gri.Poe.", "Francesco Gritti - Poesie in dialetto veneziano"),
		("Mar.Gal.Cioxa", "Mariègola dei Galafadi de Cioxa (metà XV sec.)"),
		("Mar.Cal.Cioxa", "Mariègola dei Caleghèri de Cioxa (1300-1330)"),
		("Mar.SCro.Cioxa", "Mariègola de s. Croxe de Cioxa (1387)"),
		("Mor.Chro", "Antonio Morosini - Chronica de Veniexia (1433)"),
		("Mua.Diz.", "F. Zorzi Muazzo - Dizionario veneto, voci, frasi, sentenze, proverbi"),
		("Panfilo", "Pamphilus in antico veneziano"),
		("Polo.Mil.211", "Marco Polo - Il milione veneto (CM 211 Padova)"),
		("Pro.Fem.", "Anonimo Vèneto - Proverbia quae dicuntur super natura feminarum (1152-1160-1216?)"),
		("Reg.Trev.", "Libro delle Regole del Territorio di Treviso - 1315"),
		("Ruz.Bil.", "Ruzante - Bilora"),
		("Ruz.Red.", "Ruzante - Il reduce"),
		("Ven.Str.", "M. Veniero - La strazzosa"),
		("Zan.Mis.", "A. Zanzotto - Mistieròi"),
		("Zib.Can.", "Zibaldone da Canal (XIV sec.)"),
	]);

	return res;
}

pub fn getPlacesused() -> HashMap<&'static str, &'static str> {
	let res = HashMap::from([
		("ag", "Agordin"),
		("bl", "Belun"),
		("ch", "Cioxa"),
		("ma", "Maròstega"),
		("pd", "Pàdoa"),
		("ra", "Raguxa"),
		("sd", "San Donà"),
		("sn", "Sinistra Piave"),
		("tf", "Tèra-ferma oriental"),
		("ve", "Venèsia"),
		("vi", "Vicensa"),
		("vr", "Verona"),
		("vv", "Vitòrio Vèneto"),
		("za", "ara"),
	]);

	return res;
}
