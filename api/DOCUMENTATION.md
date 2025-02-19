# Documentation

## Data Structs

`User<(id: u64, name: String)>`

An object that uniquely identifies any student.


## Endpoints

#### Getting users
<details>
<summary>
<code>GET /get_all_names</code> --> <code>Vec&lt;User&gt;</code> 
</summary>


Returns all students known to the program (Y13 students only).

</details>

#### Getting data
<details>
<summary>
<code>GET /get_data/{uuid}/{key}</code> --> <code>Vec<(String, u64)>, Vec<(User, u64)></code>
</summary>


`uuid` is the students unique id (`User.id`) and `key` is a unique code to track API usage.

`Vec<(String, u64)>` is a vector of pairs containing each classes the student takes and how many of that class is left.

`Vec<(User, u64)>` is a vector of people that share classes with the student and how many classes they each share with them.

</details>

#### Prefix searching
<details>
<summary>
<code>GET /prefix/{search}</code> --> <code>Vec&lt;User&gt;</code> 
</summary>


`search` is a prefix query for matching users.

The returned vector contains a list of users that have a loosely matching name.

</details>

#### Countdowns
<details>
<summary>
<code>GET /countdowns</code> --> <code>Vec<(String, String)></code> 
</summary>


The returned vector contains all countdowns. The first element of each pair is the countdown name, and the second is the date to be counted down to in the format `YYYY-MM-DD (H)H:MM`.

</details>

#### Connections
<details>
<summary>
<code>GET /get_connections</code> --> <code>Vec<Vec&lt;((String, String), u64)>&gt;, Vec<(User, (i32, i32, i32)></code>
</summary>


Returns a pair of vectors. The first vector contains all possible connection graphs: each element of the vector is a vector of connections. A single connection is described in the format `(String, String), u64)`, where `(String, String)` is the two ids of the users to be connected and `u64` is the strength of the connection.

The second vector contains the colour for all users in the format `(User, (i32, i32, i32))`. User is the referenced student, and the `i32` tuple describes the colour in the RGB format.

</details>


