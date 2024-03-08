# Servizi
Sono oggetti che permettono di organizzare e condividere la logica di business, i dati o le funzionalità tra diverse parti di un'applicazione.
Possono gestire la logica di accesso ai dati, la comunicazione con i server, la gestione dello stato dell'applicazione, 
Le caratteristiche sono:
- **Singleton Pattern**: viene creata un'unica istanza del servizio per l'intera applicazione. Questo permette di mantenere uno stato condiviso e di evitare la duplicazione di risorse.
- **Dependency Injection**: si forniscono i servizi alle componenti che ne hanno bisogno, le dipendenze vengono iniettate nelle classi che le richiedono, anziché essere create al loro interno. Per utilizzare un servizio in una componente Angular, è sufficiente dichiarare il servizio come dipendenza nel costruttore della componente. 
- **Riutilizzabilità del Codice**: i servizi in Angular possono essere facilmente riutilizzati in diverse parti dell'applicazione.

## Creazione

Genero un servizio di nome "housing":

	ng generate service servizi/housing

Il file TS di default sarà:

	import { Injectable } from '@angular/core';
	
	@Injectable({
		  providedIn: 'root'
	})
	export class HousingService {
		  constructor() { }
	}
	
Per andarla ad utilizzare importiamo la *HousingLocation*:

	import { Injectable } from '@angular/core';
	import { HousingLocation } from '../interfacce/housing-location';
	
	@Injectable({
	  providedIn: 'root'
	})
	export class HousingService {
	  protected housingLocationList: HousingLocation[] = [
	    {
	      id: 0,
	      name: 'Acme Fresh Start Housing',
	      city: 'Chicago',
	      state: 'IL',
	      photo: '/assets/logo.jpg',
	      availableUnits: 4,
	      wifi: true,
	      laundry: true
	    },
	    {
	      id: 1,
	      name: 'A113 Transitional Housing',
	      city: 'Santa Monica',
	      state: 'CA',
	      photo: '/assets/logo.jpg',
	      availableUnits: 0,
	      wifi: false,
	      laundry: true
	    }
	  ];
	
	  getAllHousingLocation(): HousingLocation[] {
		return this.housingLocationList;
	  }
	
	  getHousingLocationById(id : Number) : HousingLocation | undefined {
		return this.housingLocationList.find(housingLocation => housingLocation.id === id);
	  }
	  constructor() { }
	}

Il decoratore *@Injectable* dichiara che la classe *HousingService* è iniettabile come servizio. 
Il parametro *providedIn: 'root'* indica che il servizio sarà disponibile per tutta l'applicazione e verrà iniettato nel livello radice.
In questo esempio abbiamo indicato i dati direttamente dentro il servizio.
Il servizio contiene due metodi:
- **getAllHousingLocation( )**: restituisce l'intero elenco di posizioni di alloggio.
- **getHousingLocationById(id: number)**: restituisce la posizione di alloggio con l'id specificato, o undefined se non trovata.


## Utilizzo

Andiamo ora a modificare la componente Home. 
Importiamo sia *inject* che *HousingService* e cancelliamo tutti i dati precedentemente presenti nell'array *housingLocationList*
All'interno del costruttore della componente possiamo quindi invocare i metodi del servizio per importare i dati ed associarli alle variabili.

	import { Component, inject } from '@angular/core';
	import { HousingLocationComponent } from '../housing-location/housing-location.component';
	import { HousingLocation } from '../../interfacce/housing-location';
	import { CommonModule } from '@angular/common';
	import { HousingService } from '../../servizi/housing.service';
	  
	@Component({
	  selector: 'app-home',
	  standalone: true,
	  imports: [HousingLocationComponent, CommonModule],
	  templateUrl: './home.component.html',
	  styleUrl: './home.component.css'
	})
	export class HomeComponent {
	  housingLocationList: HousingLocation[] = [ ]
	  housingService: HousingService = inject(HousingService);
	
	  constructor() {
	    this.housingLocationList = this.housingService.getAllHousingLocation();
	  }
	}

## Successivo
[Forms](workshop/angular/Forms.html).