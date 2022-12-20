# Présentation du sujet et mise en place

Bienvenue pour cette démonstration du langage Rust et du Framework Axum. Ce projet a pour but de vous faire découvrir Rust et Axum à travers différentes étapes.

Pour ce projet, nous utiliserons une base de données Postgresql et nous allons apprendre à la mettre en place avec Docker dès maintenant.
Dans un premier temps, vérifiez qu'un fichier **.env** existe à la racine et qu'il contient les informations suivantes :

```
DATABASE_URL=postgres://postgres:password@localhost/postgres
POSTGRES_PASSWORD=password
POSTGRES_USER=postgres
POSTGRES_DB=postgres
```

### Descriptif de la base de données

Nous allons maintenant voir ce que contient notre base de données. Rendez-vous dans le fichier init.sql", au répertoire **db_scripts**.
Ainsi, nous pouvons voir qu'une table **tasks** va être créée si elle n'existe pas déjà, et cette table sera utilisée par sea-orm pour générer nos entités.

## 1.0 - Début du projet

### Mise en place du container de base de données

Pour mettre en place la base de données, executez les commandes suivantes en tant qu'utilisateur root.
```
docker compose up
```
En cas de problème, vous pouvez arrêter le container et supprimer la base de données à l'aide des commandes suivante :
```
docker compose down
rm -rf /data -f
```

## 1.1 - Génération des entités de la base de données avec sea-orm

Nous allons utiliser l'ORM "sea-orm" pour générer nos entités de base de données à partir de notre base de données mise en place.
Installons la dépendance de la command-line de sea-orm.
```
cargo install sea-orm-cli
```
sea-orm va utiliser notre base Posgresql, assurez-vous que le container est bien lancé.
Nous allons placer nos entités dans le répertoire "src/database".
```
sea-orm-cli generate entity -o src/database
```

## 1.2 - Cargo run

Maintenant, nous allons vérifier que le programme s'execute correctement.
```
cargo run
```

### Cargo.toml
Le programme devrait installer les dépendances présentes dans le fichier Cargo.toml. Cela permet d'importer des **crate** qui sont des librairies externes.

## 1.3 - Arborescence du projet

L'arborescence d'un projet Rust se décompose en plusieurs modules de fichiers et répertoires grâce à un fichier **lib.rs** qui a pour but de lister toutes les dépendances du projet. 

Il sera possible de définir des modules de fichiers dans des sous-arborescences grâce à des fichiers **mod.rs** qui seront eux-mêmes listés dans **lib.rs**.

Dans notre cas, nous garderons les choses légères et importantes dans des fichiers, et nous utiliserons des fichiers **mod.rs** et les sous-arborescences pour les dépendances qui peuvent tendre à s'allourdir.

### main.rs

Le fichier main.rs est le point d'entrée de notre programme. Il contient la fonction `main()` qui est appelée au lancement du programme.
Nous allons tenter de garder ce fichier le plus lisible possible, c'est pourquoi nous allons déplacer la fonction qui instancie le server `run()` dans un autre fichier **server.rs**.

**Petite note :**
`#[tokio::main]` permet d'indiquer à Rust que nous allons utiliser le runtime Tokio, qui est un runtime asynchrone.

### server.rs
Le fichier server.rs contient la fonction `run()` qui permet d'instancier le server et lancer l'application.
Mais avant cela, server.rs va permettre de définir les routes de notre application, les states ainsi que la connections en base de données.
 
### router.rs
Le fichier router.rs contient la fonction `create_routes()` qui permet de définir les routes de notre application.
C'est dans ce même routeur que nous allons définir les extensions, les middlewares, les states et les controllers.

**Petite note :**
Les routes peuvent être organisés à l'aide de **nest** pour générer des plus petit routeurs qui regroupent plusieurs routes pour un même prefix "/tasks", sur lequel esra appliqué différentes méthodes HTTP associées à différents handlers.
Les routes peuvent aussi être organisées de manière plus fines pour une gestion des autorisations par groupe d'utilisateurs.

### répertoire routes
Le répertoire routes contient les fichiers qui définissent les handlers de nos routes.

### répertoire models
Le répertoire models contient les fichiers qui définissent les structures de données.

### répertoire database
Le répertoire database contient les fichiers qui définissent les entités de la base de données généré par sea-orm.

## 1.4 - Utilitaire

### Snippets du framework Axum

Axum met à disposition des snippets pour faciliter l'apprentissage du framework.
https://github.com/tokio-rs/axum/tree/main/examples

### Debug

Vous pouvez utiliser la macro `debug!()` pour afficher des informations dans la console.
Si votre erreur est sur un handler, vous pouvez utiliser https://docs.rs/axum-macros/0.3.0/axum_macros/ pour afficher plus d'informations.

**A placer au dessus d'un handler :**
```rust
#[debug_handler]
```

## 1.5 - Création d'une stucture de données

### Avec champs nommés et options

```rust
struct Task {
    id: i32,
    priority: String,
    title: String,
    description: Option<String>,
}

let task2 = Task {
    priority: String::from("B"),
    ..task1
}
```


### Sans champs nommés

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

### Serde, Serialize, Deserialize

Serde est un framework de sérialisation et de désérialisation de données. Il permet de convertir des données structurées en un format de données sérialisé (JSON, XML, YAML, etc.) et vice versa.

Données non structurées (JSON, XML, YAML, etc.) -> Serialize 
Données structurées (struct, enum, tuple, etc.) -> Deserialize

**Il existe aussi un trait de débugage :**

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct TaskResponse {
    pub id: i32,
    pub priority: String,
    pub title: String,
    pub description: Option<String>,
}
```

## 1.6 - Création d'endpoint : GET / 

Nous allons créer notre premier endpoint.
L'endpoint doit retourner un `Result` qui doit retourner un `Ok` ou un `Err` dans lesquels seront encapsulés vos résultats.

Dans notre cas, le type retourné est une simple chaîne de caractères, nous allons donc retourner un `Ok` avec la chaîne de caractères.

https://doc.rust-lang.org/std/result/ 

**Correction :**

```rust
pub async fn hello_world() -> Result<String, (StatusCode, String)> {
    Ok(String::from("Hello World"))
}
```

## 1.7 - Création d'endpoint : GET /teapots

Pour cette endpoint, ce ne sera pas beaucoup plus compliqué que l'endpoint précédent. Cependant vous devrez retourner une structure de données.

**Correction :**

```rust
pub async fn teapot() -> Result<TeapotResponse, (StatusCode, String)> {
    Ok(TeapotResponse("I'm a teapot".to_string()))
}
```

## 1.8 - Validation de la structure de données de données

Pour valider la structure de données, nous allons utiliser le framework **validator**. Il suffit comme pour Serde d'ajouter l'annotation `#[derive(Validate)]` sur la structure de données.

https://docs.rs/validator/latest/validator/ 

**Correction :**

```rust
#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct TaskRequest {
    #[validate(length(min = 1, max = 3, message = "String must have between 1 and 3 characters"))]
    pub priority: String,
    #[validate(length(min = 3, max = 32, message = "String must have between 3 and 32 characters"))]
    pub title: String,
    #[validate(length(min = 3, max = 120, message = "Optional String must have between 3 and 120 characters"))]
    pub description: Option<String>,
}
```

Pour l'implémenter :

```rust
if let Err(errors) = request.validate() {
    return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
}
```

## 1.9 - Application state

Pour les prochaines questions, nous allons utiliser un state pour stocker notre pool de connection à la base de données et l'utiliser dans les futurs endpoints, c'est pourquoi il est important de faire un petit point.

https://docs.rs/axum/latest/axum/extract/struct.State.html

### State

Sa structure est déclarée dans le fichier app_state.rs du répertoire models et sera réutilisée à chaque utilisation du state.

```rust
use axum_macros::FromRef;
use sea_orm::DatabaseConnection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database_conn: DatabaseConnection,
}
```

Il est déclaré dans le fichier server.rs puis passer en argument à la fonction `create_routes` qui retourne un `Router`.

```rust
 let database_conn = Database::connect(database_uri).await.unwrap();
 let app_state = AppState { database_conn };
 let app = create_routes(app_state);
 ```

Ainsi, nous pouvons l'implémenter dans notre arborescence de routeur pour être utilisé dans les endpoints.

```rust
Router::new().nest("", index_nest).with_state(app_state)
```

Dans les endpoints, nous pouvons récupérer le state avec l'annotation `#[extract]` et le type `State<AppState>`.

```rust
pub async fn get_db_conn(
    State(database_conn): State<DatabaseConnection>,
) -> Result<(), (StatusCode, String)> {

}
```

## 2.0 - Création d'endpoint : POST /tasks

Nous allons maintenant créer le premier endpoint d'un CRUD sur la ressource **tasks**. Nos réponses seront de type JSON, nous allons donc utiliser le framework Serde pour sérialiser et désérialiser nos données et nous allons donc créer un nouveau fichier dans le répertoire models : **task.rs**.

Pour la première fois nous allons utiliser sea-orm pour intéragir avec notre base de données, l'ORM est très bien documenté :
https://www.sea-ql.org/SeaORM/docs/index/

Pour la première fois, nous allons aussi utiliser le routeur et devoir le modifier ajouter un nouvel endpoint, encore une fois c'est très bien documenté : https://docs.rs/axum/0.2.3/axum/routing/struct.Router.html

**Correction :**

```rust
pub async fn create_task(
    State(database_conn): State<DatabaseConnection>,
    Json(request): Json<TaskRequest>,
) -> Result<StatusCode, (StatusCode, String)> {

    if let Err(errors) = request.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
    }

    let task = tasks::ActiveModel {
        title: Set(request.title),
        priority: Set(request.priority),
        description: Set(request.description),
        ..Default::default()
    };

    let _result = task
        .save(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)));

    Ok(StatusCode::CREATED)
}
```

## 2.1 - Création d'endpoint : GET /tasks

Nous allons maintenant créer un endpoint qui va nous permettre de récupérer toutes les tâches de la base de données.

**Correction :**

```rust
pub async fn get_all_tasks(
    State(database_conn): State<DatabaseConnection>,
) -> Result<Json<Vec<TaskResponse>>, (StatusCode, String)> {

    let tasks = tasks::Entity::find()
        .all(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?
        .into_iter()
        .map(|db_task| TaskResponse {
            id: db_task.id,
            title: db_task.title,
            priority: db_task.priority,
            description: db_task.description,
        })
        .collect();

    Ok(Json(tasks))
}
```

## 2.2 - Création d'un filtre sur l'endpoint : GET /tasks

Nous allons maintenant créer un endpoint qui va nous permettre de récupérer toutes les tâches de la base de données en fonction d'un filtre.

Pour cela nous allons utiliser QueryParams : https://docs.rs/axum/latest/axum/extract/struct.Query.html pour récupérer un paramètre optionnel.

**Correction :**

```rust
pub async fn get_all_tasks(
    State(database_conn): State<DatabaseConnection>,
    query_params: Option<Query<GetTaskQueryParams>>,
) -> Result<Json<Vec<TaskResponse>>, (StatusCode, String)> {
    let priority_filter = match query_params {
        Some(query_params) => {
            if let Err(errors) = query_params.validate() {
                return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
            }
            Condition::all().add(tasks::Column::Priority.eq(&*query_params.priority))
        }
        None => Condition::all(),
    };

    let tasks = tasks::Entity::find()
        .filter(priority_filter)
        .all(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?
        .into_iter()
        .map(|db_task| TaskResponse {
            id: db_task.id,
            title: db_task.title,
            priority: db_task.priority,
            description: db_task.description,
        })
        .collect();
        
    Ok(Json(tasks))
}
```

## 2.4 - Création d'endpoint : GET /tasks/{id}

Cette fois nous allons devoir récupérer le path de l'endpoint, pour cela nous allons utiliser l'annotation `#[extract]` et le type `Path<i32>`.

Quelques exemples : https://docs.rs/axum/latest/axum/extract/index.html

**Correction :**

```rust
pub async fn get_task(
    Path(id): Path<i32>,
    State(database_conn): State<DatabaseConnection>,
) -> Result<Json<TaskResponse>, (StatusCode, String)> {
    let task = tasks::Entity::find_by_id(id)
        .one(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)));

    match task.unwrap() {
        Some(task) => {
            let task = TaskResponse {
                id: task.id,
                title: task.title,
                priority: task.priority,
                description: task.description,
            };
            Ok(Json(task))
        }
        None => Err((StatusCode::NOT_FOUND, "Task not found".to_string())),
    }
}
```

## 2.5 - Création d'endpoint : DELETE /tasks/{id}

Nous allons maintenant créer un endpoint qui va nous permettre de supprimer une tâche de la base de données, nous récupérerons l'id de la tâche depuis le path comme pour l'endpoint précédent.

**Correction :**

```rust
pub async fn delete_task(
    Path(id): Path<i32>,
    State(database_conn): State<DatabaseConnection>,
) -> Result<(), (StatusCode, String)> {
    let task: Option<tasks::Model> = tasks::Entity::find_by_id(id)
        .one(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?;

    if let Some(task) = task {
        let task: tasks::Model = task.into();
        let res: DeleteResult = task
            .delete(&database_conn)
            .await
            .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?;
        assert_eq!(res.rows_affected, 1);
        Ok(())
    } else {
        Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
    }
}
```

## 2.6 - Création d'endpoint : PUT /tasks/{id}

Nous allons maintenant créer un endpoint qui va nous permettre de mettre à jour une tâche de la base de données, nous récupérerons l'id de la tâche depuis le path comme pour l'endpoint précédent.

**Correction :**

```rust
pub async fn update_task(
    Path(id): Path<i32>,
    State(database_conn): State<DatabaseConnection>,
    Json(request): Json<TaskRequest>,
) -> Result<Json<TaskRequest>, (StatusCode, String)> {
    if let Err(errors) = request.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
    }

    let task: Option<tasks::Model> = tasks::Entity::find_by_id(id)
        .one(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?;

    match task {
        Some(task) => {
            let mut task: tasks::ActiveModel = task.into();

            task.title = Set(request.title);
            task.priority = Set(request.priority);
            task.description = Set(request.description);

            let task: tasks::Model = task
                .update(&database_conn)
                .await
                .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?;

            Ok(Json({
                TaskRequest {
                    title: task.title,
                    priority: task.priority,
                    description: task.description,
                }
            }))
        }
        None => Err((StatusCode::NOT_FOUND, "Task not found".to_string())),
    }
}
```