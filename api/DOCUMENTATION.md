# Documentation

## Data Structs

`User<(id: u64, name: String)>`

An object that uniquely identifies any student.


## Endpoints

#### Getting users
<details>
<summary>
`GET /get_all_names` --> `Vec<User>` 
</summary>

Returns all students known to the program (Y13 students only).

</details>

#### Getting data
<details>
<summary>
`GET /get_data/{uuid}/{key}` --> `Vec<(String, u64)>, Vec<(User, u64)>` 
</summary>

`uuid` is the students unique id (`User.id`) and `key` is a unique code to track API usage.

`Vec<(String, u64)>` is a vector of pairs containing each classes the student takes and how many of that class is left.

`Vec<(User, u64)>` is a vector of people that share classes with the student and how many classes they each share with them.

</details>

#### Prefix searching
<details>
<summary>
`GET /prefix/{search}` --> `Vec<User>` 
</summary>

`search` is a prefix query for matching users.

The returned vector contains a list of users that have a loosely matching name.

</details>

#### Countdowns
<details>
<summary>
`GET /countdowns` --> `Vec<(String, String)>` 
</summary>

The returned vector contains all countdowns. The first element of each pair is the countdown name, and the second is the date to be counted down to in the format `YYYY-MM-DD (H)H:MM`.

</details>

#### Connections
<details>
<summary>
`GET /get_connections` --> `Vec<Vec<((String, String), u64)>>, Vec<(User, (i32, i32, i32))>` 
</summary>

Returns a pair of vectors. The first vector contains all possible connection graphs: each element of the vector is a vector of connections. A single connection is described in the format `(String, String), u64)`, where `(String, String)` is the two ids of the users to be connected and `u64` is the strength of the connection.

The second vector contains the colour for all users in the format `(User, (i32, i32, i32))`. User is the referenced student, and the `i32` tuple describes the colour in the RGB format.

</details>


