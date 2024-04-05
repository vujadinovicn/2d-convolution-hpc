# High Performance Computing on 2D convolution block

#### Osnovne informacije
Student: Nemanja Vujadinović SV28/2020  
Projekat će biti implementiran za ocenu 10.

#### Opis problema
Projekat se fokusira na implementaciju 2D konvolucionog bloka. Konvolucioni blok je potporna osnova većine neuronskih mreža. Blok sadrži konvolucioni sloj (_Convolutional layer_), propraćen aktivacionom funkcijom (_Activation function_), a dodatno može sadržati slojeve za normalizaciju (npr. _Batch norm layer_ ili _Dropout layer_) i slojeve za sažimanje (npr. _Max pooling_ ili _Average pooling_).
Naš konvolucioni blok predstavlja uzastopnu primenu sledećih koraka:
- Konvolucioni sloj - 2D konvolucija sa random generisanim filterima
- Aktivaciona funkcija - primena aktivacione funkcije (ReLU) na matricu izlaza iz konvolucionog sloja 
- Maksimalno sažimanje - smanjivanje dimenzionalnosti matrice primenom maksimalnog sažimanja

U slučaju neuronskih mreža za rad sa slikama, ulazne slike su obično RGB - imaju tri kanala. Slika prolazi kroz veći broj (složenih) konvolucionih blokova i izlaz iz mreže pruža smislen rezultat - klasifikaciju slike, vrednost dobijenu regresijom, segmentiranu sliku... 
S obzirom da izlaz iz jednog konvolucionog bloka ne predstavlja značajnu vizuelnu reprezentaciju, odlučeno je da se algoritam primeni na _random_ generisanu matricu sa jednim kanalom. Ideja projekta leži u analizi i poređenju performansi prilikom izvođenja operacija nad matricama, umesto fokusa na evaluaciju izlaza iz konvolucionog bloka.

#### Opis implementacije
Kao što smo već naveli, ulaz u konvolucioni blok će biti _random_ generisane slike. Biće potrebno koristiti matrice različite veličine, te je ideja da se sve te matrice, kao i filter matrice, na početku nasumično generišu i sačuvaju u fajlove kako bi sekvencijalno i paralelno rešenje u oba jezika (Python i Rust) radile sa istim podacima.  
Ulazna matrica prvo "prolazi" kroz konvolucioni sloj. Ovo podrazumeva množenje podmatrica ulazne matrice sa filter matricom. Podmatrica ulazne matrice su iste veličine kao filter matrica. Filter se kreće korak po korak (biće dodatno objašnjeno u delu za paralelizam). Za svaki korak, odgovarajući elementi filtera se množe sa odgovarajućim elementima podmatrice ulazne matrice. Rezultati _element-wise_ množenja se sumiraju kako bi se dobio jedan rezultujući element izlazne matrice. Nakon svakog koraka, filter se pomiče za određeni korak u horizontalnom i vertikalnom smeru kako bi se pokrila cela ulazna matrica. Razmisliće se o ideji popunjavanja ivica nulama (_padding_) kako bi izlazna matrica bila iste veličine kao ulazna. Postupak se ponavlja dok filter ne pokrije celu ulaznu podmatricu.  
Nad izlazom iz konvolucionog sloja, primenjujemo aktivacionu funkciju - ReLU. ReLU funkcija je oblika `
f(x) = max(0, x)
`. Funkcija se primenjuje nad svakim elementom matrice.  
Konačno, primenjujemo maksimalno sažimanje. Veličina filtera za maksimalno sažimanje biće naknadno i eksperimentalno određena. Za svaku podmatricu ulazne matrice koja je iste veličine kao i filter matrica traži se maksimalna vrednost. Ova maksimalna vrednost će biti zadržana u izlaznoj matrici, dok će ostale vrednosti biti odbačene. Na ovaj način, smanjuje se dimenzija matrice. 

#### Sekvencijalno rešenje
Sekvencijalno rešenje biće implementirano po ugledu na prethodno objašnjen algoritam. Menjaće se veličine ulaznih matrica ili veličine filtera i za takve različite slučajeve, beležiće se performanse. Rešenja će biti implementirana u jezicima Python i Rust. 

#### Paralelno rešenje
Trenutna ideja za paralelno rešenje je da koristi prethodno pomenutu implementaciju, ali nad izdeljenim matricama. Ovo bi značilo da u slučaju 4 paralelna procesa, matricu podelimo na 4 podmatrice, koje će imati veličinu N/4 x N/4, gde je N veličina ulazne matrice. Na ovaj način delimo i ubrzavamo posao. Takođe će se menjati veličine matrica kao i u prethodnom slučaju.  
Kao što smo već spomenuli, korak (_stride_) predstavlja pomeraj filtera po x i y osi matrice. Ovaj deo može igrati bitnu ulogu pri paralelizaciji rešenja. Ako želimo da ulaznu matricu izdelimo na P matrica (gde je P broj procesa) i tako ih prepustimo procesima, važno je da se konvolutivni filter ne primeni na podmatricu (iste veličine kao filter) sa elementima koji pripadaju različitim izdeljenim matricama. Ovaj negativan slučaj može da se desi u centru matrice ili na drugim granicama izdeljenih matrica. Želimo da izbegnemo ovaj slučaj kako bi procesi radili nezavisno. Zbog toga će biti potrebno odabrati dobar korak (pomeraj). Osim koraka, potrebno je da obratimo pažnju i na veličinu filtera za maksimalno sažimanje. Trenutna pretpostavka je da će ovo moći biti rešeno popunjavanjem ivica matrice, korišćenjem ulaznih matrica kvadratnih dimenzija i korišćenjem kvadratnih filtera čija dimenzija je delilac dimenzije ulazne matrice. 

Rešenja će biti implementirana u jezicima Python (korišćenjem _multiproccessing_ biblioteke) i Rust (korišćenjem niti). 

#### Eksperimenti jakog i slabog skaliranja
Implementiraćemo eksperimente jakog i slabog skaliranja kako bi se uporedili dobijeno ubrzanje paralelizovane Python/Rust implementacije u odnosu na sekvencijalnu implementaciju istog jezika. Kao što je objašnjeno u sekciji _Paralelno rešenje_, pretpostavlja se da će veličina matrice i broj procesa biti međusobno zavisni, te će se spram brojem procesa menjati i veličina matrice.

#### Vizualizacija rešenja
Rešenje će biti vizualizovano pružanjem informacija i rezultata u slučaju:
- sekvencijalnog/paralelnog algoritma korišćenjem jezika Python/Rust. Menjaće se veličine ulazne matrice ili veličine konvolutivnih filtera. Za svaki slučaj biće prikazano vreme na y osi, a veličina matrice na x osi. 
- slabog/jakog skaliranja za poređenje sekvencijalnog i paralelnog algoritma korišćenjem jezika Python/Rust u skladu sa Amdalovim i Gustafsonovim zakonom.
Za vizuelizaciju Python rešenja, koristiće se biblioteka `matplotlib`. Za vizuelizaciju Rust rešenja, koristiće se jedna od dve biblioteke - `Plotters` i `Rerun`.
