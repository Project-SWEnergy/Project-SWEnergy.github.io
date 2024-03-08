Gestisce la navigazione tra componenti.
Consente di definire percorsi per diverse viste e di mappare queste viste a determinati URL. 
Ciò significa che quando un utente cambia l'URL o esegue un'azione che dovrebbe portarlo a una vista diversa, l'applicazione Angular può rispondere navigando in modo appropriato senza dover ricaricare l'intera pagina.

# Creazione
## Modulo

Per utilizzarlo è necessario abilitarlo, il tutorial dice di inserire la componente dentro *main.ts*, tuttavia dovrebbe essere già abilitato all'interno del file *appConfig* in esso incluso:

	import { ApplicationConfig } from '@angular/core';
	import { provideRouter } from '@angular/router';
	import { routes } from './app.routes';
	
	export const appConfig: ApplicationConfig = {
	  providers: [provideRouter(routes)]
	};

## Routes

Nel file *app.routes.ts* è importante definire il parametro *Routes* (anche questo definito in automatico).
Le routes definiscono la mappatura tra un percorso URL e un componente specifico. 
L'interno di default di tale file è il seguente:

	import { Routes } from '@angular/router';
	export const routes: Routes = [];
	
Ora vogliamo inserire i dati relativi a quanto fatto fino ad ora nel tutorial, lo modifichiamo quindi come segue:

	import { Routes } from '@angular/router';
	import { HomeComponent } from './componenti/home/home.component';
	
	export const routes: Routes = [
	    {
	        path: '',
	        component: HomeComponent,
	        title: 'Home Page'
	    }
	];

## RouterOutlet

Un'applicazione Angular deve includere un < router-outlet > nel suo template principale. Questo è il punto in cui le viste corrispondenti ai percorsi specificati vengono visualizzate dinamicamente.
Dal file *app.component.ts* importiamo il modulo *RouterModule* da *@angular/router*.

	import { Component } from '@angular/core';
	import { CommonModule } from '@angular/common';
	import { RouterOutlet, RouterModule } from '@angular/router';
	import { HomeComponent } from './componenti/home/home.component'
	
	@Component({
		  selector: 'app-root',
		  standalone: true,
		  imports: [
		    CommonModule,
		    RouterOutlet,
		    RouterModule,
		    HomeComponent],
		  templateUrl: './app.component.html',
		  styleUrl: './app.component.css'
		})
	export class AppComponent {
		  title = 'LearningAngular';
	}

Modifichiamo ora il file *app.component.html* inserendo il tag relativo al routing al posto di quello < app-home > come segue:

	<main>
		  <header class="brand-name">
		    <img class="brand-logo" 
		      src="../assets/logo.jpg"
		      alt="logo"
		      aria-hidden ="true">
		  </header>
		  <section class="content">
		    <router-outlet></router-outlet>
		  </section>
	  </main>

Deve essere ben definito il routing della sezione [[#Routes]]

## Router Link

Creo un componente chiamato *details* che dovrà apparire alla selezione di una delle cards presenti nel progetto.
Aggiungiamo in *app.routing.ts* la componente tramite import e la definiamo così come è stata definita la Home:

	import { Routes } from '@angular/router';
	import { HomeComponent } from './componenti/home/home.component';
	import { DetailsComponent } from './componenti/details/details.component';
	
	export const routes: Routes = [
	    {
	        path: '',
	        component: HomeComponent,
	        title: 'Home Page'
	    },
	    {
	        path: 'details',
	        component: DetailsComponent,
	        title: 'Details'
	    }
	];

Ci posizioniamo ora nella componente che visualizza le cards ed inseriamo un'ancora alla componente di dettaglio, importiamo il *RouterModule*.
Nel suo file HTML inseriamo il routerLink come segue:

	<section class="listing">
	    <img class="listing-photo" 
	        [src]="housingLocation.photo"
	        alt=" Foto di {{housingLocation.name}}">
	    <h2 class="listing-heads"> {{ housingLocation.name}} </h2>
	    <p class="listing-location"> {{housingLocation.state}} </p>
	    <p routerLink="details">Learn more!</p>
	</section>  

Questo codice permette di connettersi ad una nuova pagina/componente, in questo caso però lo stesso pulsante è presente su diverse cards e noi vogliamo connetterci ad una specifica pagina.
Per gestire la situazione modifichiamo il routerLink effettuando un binding con le proprietà della card:

	<section class="listing">
	    <img class="listing-photo" 
	        [src]="housingLocation.photo"
	        alt=" Foto di {{housingLocation.name}}">
	    <h2 class="listing-heads"> {{ housingLocation.name}} </h2>
	    <p class="listing-location"> {{housingLocation.state}} </p>
	    <p [routerLink]="['/details', housingLocation.id]">Learn more!</p>
	</section>  

Occorre però modificare anche la pagina di routing, indicheremo con i : un placeholder per un parametro che dovrà essere passato al momento del click come segue:

	import { Routes } from '@angular/router';
	import { HomeComponent } from './componenti/home/home.component';
	import { DetailsComponent } from './componenti/details/details.component';
	
	export const routes: Routes = [
	    {
	        path: '',
	        component: HomeComponent,
	        title: 'Home Page'
	    },
	    {
	        path: 'details/:id',
	        component: DetailsComponent,
	        title: 'Details'
	    }
	];

## Inject

Ora che abbiamo aperto un dettaglio della card occorre poter ricevere nella componente il parametro inviato nell' url (l'ID).
Modifichiamo la componente di dettaglio inserendo i moduli necessari a ricevere i dati e creando il costruttore della componente in modo che recepisca il valore e lo assegni alla variabile:

	import { Component, inject } from '@angular/core';
	import { ActivatedRoute } from '@angular/router';
	
	@Component({
	  selector: 'app-details',
	  standalone: true,
	  imports: [],
	  templateUrl: './details.component.html',
	  styleUrl: './details.component.css'
	})
	export class DetailsComponent {
	  route: ActivatedRoute = inject(ActivatedRoute);
	  housingLocationId = 0;
	
	  constructor() {
	    this.housingLocationId = Number(this.route.snapshot.params["id"])
	  }
	}

# Successivo
[[Services]]