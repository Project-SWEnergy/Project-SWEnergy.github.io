Utilizzate per creare interfacce utente modulari e riutilizzabili. 
Ogni componente rappresenta una parte specifica dell'interfaccia utente o della logica dell'applicazione.


# Descrizione

Un esempio è quello principale in *app.component.ts*:

	import { Component } from '@angular/core';
	@Component({
		 selector: 'app-root',
		 templateUrl: './app.component.html',
		 styleUrls: ['./app.component.css'],
	})
	export class AppComponent {
		 title = 'TestApplication';
	}
	
- **selector**: Selettore che identifica il tag utilizzato per invocare il componente all'interno di una pagina. L'utilizzo del tag della pagina principale è visionabile su *src/index.html.*
- **templateUrl**: Percorso del file HTML che contiene il template della componente. Il template è la struttura visiva della componente.
- **styleUrls**: Array di percorsi per i file di stile CSS associati a questa componente. Gli stili definiti qui saranno applicati solo alla componente corrente.

# Creazione

Creo la componente *product-list* che conterrà una lista di prodotti.

	ng generate component componenti/Home

Vengono generati i file HTML, CSS, TS relativi a tale componente.
Viene aggiornato automaticamente il file app.module.ts, che contiene tutti gli elementi del progetto.

Per includere tale componente in un file html è necessario inserire il tag ad esso dedicato ma anche informare il file dell'esistenza di tale componente.
Per farlo andiamo su *app.component.ts* e aggiungiamo il seguente import:

	import { HomeComponent } from './componenti/home/home.component'

Mentre nel decorator *@Component* la inseriamo negli import:

	imports: [CommonModule, RouterOutlet, HomeComponent]

Di default queste componenti sono definite "Standalone" in quanto non necessitano di moduli per il loro funzionamento.


# Successivo
[[Interfacce]]