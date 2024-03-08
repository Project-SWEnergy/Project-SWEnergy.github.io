# Intro

Requisiti:
- Node.js -> npm incluso in Node.js
- VS Code con estensioni standard di Angular
	- Angular Language Service
	- Angular Snippets
	- Angular Evergreen
	- Rename Angular Component


## Installazione con comando:

Installazione di Angular:

	npm install -g @angular/cli

Per controllare la versione installata:

	ng --version
  
Installiamo anche TypeScript globalmente con il comando:

	npm install -g typescript

Se si vogliono effettuare test per chiamate HTTP seguire la procedura di installazione di [[HTTP]].


## Creazione nuovo progetto:

    cd \*Percorso di destinazione\*
	ng new Nome_Applicazione
	
Il comando crea una applicazione di default.

Selezionare CSS ed accettare (?) il Server Side Rendering.
[#todo]


## Compilazione

Se si è scaricato un progetto esistente da git sarà necessario installare tutte le dependencies necessarie, già descritte nel pacchetto scaricato, per farlo usiamo il seguente comando all'intenro della cartella in cui è stato estratto il pacchetto:

	npt install

Se si dispone di TypeScript installato correttamente è possibile compilare tutti i file ts in file js dopo averli modificati, per farlo si usa il comando:

	tsc
	
L'output è presente di default nella cartella *dist/out-tsc*, tale cartella è modificabile da dentro il file *tsconfig.json* alla voce *outDir*.

TypeScript non impedisce la compilazione dei file, semplicemente genera errori e li notifica.


## Contenuto progetto vuoto

- **angular.json**: dati di progetto, di compilazione, internazionalizzazione.
- **package.json**: contiene le dependencies
- **tsconfig.json:** configurazioni di TS.
- **src**:
	- **app**:
		- **app.component**: componente di base dell'applicazione, divisa in CSS, TS, HTML.
	- **index.html**: file della pagina principale dell'applicativo, contiene il tag della componente principale < app-root >


## Avvio applicazione

    ng serve

Ci si collega al sito tramite http://localhost:4200.
Da VSCode aprire un nuovo terminale per eseguire altri comandi e visualizzare le modifiche in realtime.


# Successivo
- Per informazioni sul linguaggio continua su [[TypeScript]].
- Per continuare con Angular [[Componenti]]