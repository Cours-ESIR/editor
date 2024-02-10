#import "template/ESIR.typ": *
#let title = "Compte Rendu"
#let author = "MOREAU de L. Nicolas\nMAIRE-AMIOT Tanguy \nMERIEN Mathieu\n LE DILAVREC Titouan"
#let course_id = "INNOV-RECH"
#let instructor = "LEBULLENGER Ronan"
#let semester = "Semestre 5"
#let due_time = "XX"
#set enum(numbering: "a)")
#show: assignment_class.with(title, author, course_id, instructor, due_time)
#show link: underline
#set par(justify: true)

#set page(footer: [
  #h(1fr)
  #counter(page).display(
    "1/1",
    both: true,
  )
])

#import "@preview/codelst:2.0.0": sourcecode
#import "@preview/algo:0.3.3": algo, i, d, comment, code

#outline(
    title: [Sommaire],
    indent: auto,
    depth: 2
)

= Projet: Ballon-Sonde

// LOGO 
#figure(image("image/logo inoov.png", width: 40%), caption : [logo projet])
// TODO Nom Projet

Notre projet est d'envoyer un ballon-sonde à haute altitude (> 15km). Le but sera d'effectuer des messure de différents paramètres. Par exemple: 
- La température en fonction de l'altitude, et en particulier, étudier les mouvements des courants d'air à différentes températures en altitude ;
- Les mouvement des masses d'air en haute altitude ;
- La teneur en ozone dans l'atmosphére.

Nos différentes mesures pourront permettre de comparer nos résultats avec les prévisions des différents modèles météorologiques, ainsi que les modèles de prédiction de trajectoire de notre ballon. Cela servira à vérifier la véracité de ces modèles.

Nous avons également choisi ce projet car celui-ci nous paraissait réalisable, challengeant, et intéressant à la fois, ce qui nous motive.

= Etat de l'art 

== Organisation et Acteur français

=== CNES 

Le CNES Finance, dévelope et met en place le systeme national de ballon français pour les science et les technologie. Le CNES est donc un acteur de premier choix en terme de recherche et de dévelopment de ballon sonde.

=== Industrie

On à trouver plusieur entreprise qui produisse les different equipement et sous-systeme des ballon sonde. HEMERIA produit les envelopes des ballon. 
ELTA, MICROTEC, ADENEO, EREMS, CROSSWAY produissent des systémes embarqué pour les sonde, CAP GEMINI (partie terrestre)

=== Entreprise Commercial 
- Lux Aerobot: (https://www.luxaerobot.com/technology/)
Entrepise Englaise qui produit des ballon et des plateforme pour des equipement d'observation 

- Strato Flight (www.stratoflights.com)
Propose des vole en ballon de plus de 40 000m d'altitude. Fait des video publicitaire d'objet pour une marque dans la stratosphère.

== Objectifs des ballons sondes

=== Météorologie

@durui2014 Dans les années 1980, les chercheurs ont commencé à observer une diminution de la teneur en ozone dans la stratosphère. Bien que depuis une étude en 2014 on sait que la couche d'ozone se rétabli, son retour à la normale reste un sujet d'étude. Les processus chimiques influençant l'ozone sont aujourd'hui bien connus, cependant, le mouvement des masses d'air et les phénomènes météorologiques restent encore une zone d'ombre à cette altitude. Les intéractions entre la troposphère et la stratosphère ont pourtant une influence sur des phénomènes qui restent incompris dans la couche de zone, comme des appauvrissement soudins et ponctuels. De plus, la méconaissance de la dynamique stratosphérique limite les modèles météos actuels, la stratosphère souffre d'un manque flagrant d'informations. Hertzog et al. (2004) ont utilisé des ballons pour sonder la basse stratosphère, le mouvement de ces ballons permettaient de connaître le mouvement des vents à cette altitude avec une erreur relativement faible. Les travaux de recherche de F.Duruisseau en 2015 ont permit grâce aux données de vol des ballons de vérifier la véracité des modèles météorologiques actuels et d'étudier les intéractions entre la troposphère et la stratosphère. 

=== Rayonnement cosmique

@catal2015
Depuis que Victor Hess a découvert les rayons cosmiques en 1912, nous n'avons pas découvert l'origine de ces rayons. Les rayons cosmiques qui intriguent les chercheurs sont les rayons à ultra-haute énergie (RCUHES), avec une énergie de l'ordre de 10^20eV. Ces rayons sont très rares, ils n'aparaissent en moyenne que une fois par km^2 par siècle. Les ballons stratosphériques permettent de tester du matériel d'obvervation de ces rayons qui sera ensuite installé sur des instruments spaciaux. 

== Veille Technologique

=== types de ballon
// Mathieu
 @type_ballon Il existe 4 grands types de ballon adapté à différents besoins. Tout ces ballons marche avec le même principe physique. Ces types de ballon sont:
- les ballons ouverts
- les ballons fermés dilatatable
- les ballons fermés pressurisés
- les ballons captifs

par ordre d'altitude max
- les ballons captifs sont des ballons restant proche du sol avec 2 sous-partie, les ballons captifs, reliée à un câble (captif) et l'aéroclipper qui dérive au gré des vents au ras du sol.
- Les ballons pressurisés, ces ballons ont la capacité de faire des vols de plusieurs mois.
Ils évoluent entre 18 à 20 km.
- les ballons dilatables, ce ballon est essentiellement utilisé pour la météorologie. Ce ballon éclate entre 20 et 30km d'altitude
- les ballons ouverts peuvent voler jusqu'à 40km d'altitude.
Ils sont les plus utilisés. Comme les ballons météo, leur envoler dure une journée, mais contrairement à celui-ci, le ballon n'explose pas et est réutilisable.

=== Structure Détaillée d'un ballon
Outre un résumé sur qu’est-ce qu’un ballon stratosphérique (ballon qui peut aller de 15 à 45 Km d’altitude), cet article nous présente la structure détaillée de ce type de ballon . Sa composition est séparé en 9 parties : 
- Le clapet : il permet de contrôler l’altitude en relâchant du gaz.
- L’enveloppe : le contenant qui va pouvoir accueillir le gaz
- La nacelle enveloppe : elle contient un transpondeur, un récepteur et un feu à éclat. Cette partie permet de contenir les éléments obligatoires à la localisation du ballon pendant et après son lancement.
- Le séparateur télécommandé : permet de séparer le ballon et la chaîne de vol
- Le parachute : il va ralentir la chute de la chaîne de vol
- La nacelle de servitude opérationnelle : contient tous les récepteurs utilisés
- La nacelle antenne bande S : permet de garder le lien radio entre le ballon et nous
- La nacelle feu à éclats : contient un feu à éclats pour que le ballon soit visible la nuit
- La nacelle scientifique : contient l’ordinateur de bord, le système d’alimentation, moteur et des capteurs pour s’orienter

Ainsi cet article permet de voir les principales parties que doit contenir notre ballon. Chacune de ses parties doit être étudiée pour pouvoir optimiser la prise de mesure ainsi que simplifier la descente. De plus, la nacelle enveloppée et le feu à éclats nous introduisent des éléments obligatoires à mettre dans nos ballons. Cela va nous pousser à nous renseigner sur toute la renseignement concernant le lancement d’un ballon.


=== A balloon trajectory prediction system 

@harstad2012analysis @MUSSO20041722 La prédiction du ballon-sonde nécessite la collecte de données en temps réel a la fois des données atmoshérique/météorologique de service tierce ainsi que la collecte des donnée de position, de vitesse, d'altitude et de vent qui affecte la sonde.   
La précision de la trajectoire du ballon est devenu nécessaire pour respecter les condition de sécurité. Pour palier a des défaillance matérielle, il nous faudra pouvoir aussi prédire la trajactoire du ballon en cas de perte de communication. 

@Clark_2019 Nous avons aussi trouvé un outils qui permat de traker, self-destruct et recover pour les ballon-sonde. Cette outils à été utliser de nombreuse fois. On pourra utliser cette ressource pour concevoir notre propre systeme de tracking, communication, etc..  

=== Pouvoir contrôler la trajectoire de descente

Un des points importants de notre projet est de pouvoir contrôler la trajectoire de la descente du ballon-sonde. Là où les entreprises cherchent à prédire la trajectoire du ballon, pouvoir jouer sur sa descente nous permettrait à la fois de nous éviter de nous déplacer des kilomètres pour récupérer notre ballon et aussi cela permettrait à ce que notre ballon n'atterrit pas dans un endroit non voulu (rivière, propriété privé, etc…). C’est l’idée qu’à eu 5 étudiants de l’école EPFL. Ils voulaient pouvoir contrôler l’ouverture et la rétraction du parachute pour optimiser la prise au vent. Pour cela, le système repose sur un code automatisé qui prend en compte les informations météorologiques comme le vent, ainsi que des indications GPS. Leur mécanisme pouvant contrôler le parachute à fonctionné, toutefois ils ont eu un problème au niveau de la puissance du microcontrôleur qui n’a pas permis d’enregistrer toutes les données voulues.
En conclusion, cet article permet de nous introduire le fait de pouvoir contrôler le parachute lors de la descente de manière automatique ainsi que certains des facteurs que nous devons prendre en compte. De plus, il nous met en garde quant à la puissance des moteurs et microcontrôleurs dans le ballon.

== Veille juridique

=== règlementation régissant un ballon sonde
// Mathieu

@europ_regl il existe 3 catégories de ballon
les ballons 
lourd : charge utile de 6 kg   ou   un lot de 3kg   ou   force pour rompre le $"câble" > 230 N$


moyen : pas lourd et masse utile $4 "kg" < "masse" <6 "kg"$
léger $"masse" < 4 "kg"$


Pour un ballon lourd, il y a une règlementation plus stricte :
il faut arrêter le ballon si la visibilité est trop faible en dessous de 18km d'altitude.
Il doit avoir 2 dispositifs indépendants pour "mettre fin au transport de la charge utile" ainsi qu'un dispositif pour être détecté par un radar.
On ne peut l'envoyer si l'équipement de détection au sol n'est pas satisfaisant (SSR ou ADS-B).
Un ballon ou la force pour rompre le câble est supérieur à 230N doit avoir des fanions de couleur tous les 15m sur le câble.
Le ballon doit avoir un balisage lumineux pour voler de nuit en dessous de 18km d'altitude ainsi qu'un parachute de couleur vive en cas d'utilisation de nuit.

Il faut l'autorisation de l'état pour le lancement du ballon, le survol du territoire.\
Il faut donner régulièrement les informations de vol aux autorités.

En cas de problème ou d'infraction du règlement, il faut arrêter le vol.

= Mise en Place du Projet

== Calendrié 

- S6: Preparation et conception de la sonde/ charge utilise .
- S7: Préparation du ballon avec la sonde et tout l'equipement necessaire + debut de la demande de lancé
- S8: finalisation du projet, lancement.


== Liste d'idée retenu


=== Réutilisation du ballon
// éviter que le ballon explose.
// Problème éventuelle prise au vent lors de la chute du ballon et donc plus de distance de l'origine du décollage.
// Existe deja

=== liste d'idée supplémentaire
-  Faire un ballon stratosphérique ou l'enveloppe est aussi un parachute pour avoir un ballon 100% réutilisable. \
-  Réussir de guidé la trajectoire de descente du ballon grâce aux fils du parachute. \


#pagebreak()

#bibliography((
  "bibliographie/ABalloonTrajectoryPredictionSystem.bib",
  "bibliographie/EusoBallon.bib",
  "bibliographie/analyseVent.bib",
  "bibliographie/prediction.bib",
  "bibliographie/SondeMeteo.bib",
  "bibliographie/open_source_toolkit.bib",
  "bibliographie/AProposDesBallons.bib",
  "bibliographie/type_ballon.bib",
  "bibliographie/reglement_europeen.bib",
  "bibliographie/autorisation.bib"
 ) 
, style: "ieee", full: true)

