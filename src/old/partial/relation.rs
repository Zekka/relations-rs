// == interface ==
// -- one to one --
trait ReadOneToOneEdge<A, Edge, B> {

}

trait OneToOneEdge<A, Edge, B>: ReadOneToOneEdge<A, Edge, B> {

}

// -- one to many --
trait ReadOneToManyEdge<A, Edge, B> {

}

trait OneToManyEdge<A, Edge, B>: ReadOneToManyEdge<A, Edge, B> {

}

// -- many to many --
trait ReadManyToManyEdge<A, Edge, B> {

}

trait ManyToManyEdge<A, Edge, B>: ReadManyToManyEdge<A, Edge, B> {

}

// == shorthand ==
// -- one to one --
trait ReadOneToOne<A, B> {

}

impl<A, B> ReadOneToOne<A, B> for ReadOneToOneEdge<A, (), B> {

}

trait OneToOne<A, B> {

}

impl<A, B> OneToOne<A, B> for OneToOneEdge<A, (), B> {

}

// -- one to many --
trait ReadOneToMany<A, B> {

}

impl<A, B> ReadOneToMany<A, B> for ReadOneToManyEdge<A, (), B> {

}

trait OneToMany<A, B> {

}

impl<A, B> OneToMany<A, B> for OneToManyEdge<A, (), B> {

}

// -- many to many --
trait ReadManyToMany<A, B> {

}

impl<A, B> ReadManyToMany<A, B> for ReadManyToManyEdge<A, (), B> {

}

trait ManyToMany<A, B> {

}

impl<A, B> ManyToMany<A, B> for ManyToManyEdge<A, (), B> {

}

// == implementation ==
// -- one to one --
struct OneToOneStruct<A, Edge, B> {

}

impl <A, Edge, B> ReadOneToOneEdge<A, Edge, B> for OneToOneStruct<A, Edge, B> {

}

impl <A, Edge, B> OneToOneEdge<A, Edge, B> for OneToOneStruct<A, Edge, B> {

}

// -- one to many --
struct OneToManyStruct<A, Edge, B> {

}

impl <A, Edge, B> ReadOneToManyEdge<A, Edge, B> for OneToManyStruct<A, Edge, B> {

}

impl <A, Edge, B> OneToManyEdge<A, Edge, B> for OneToManyStruct<A, Edge, B> {

}

// -- many to many --
struct ManyToManyStruct<A, Edge, B> {

}

impl <A, Edge, B> ReadManyToManyEdge<A, Edge, B> for ManyToManyStruct<A, Edge, B> {

}

impl <A, Edge, B> ManyToManyEdge<A, Edge, B> for ManyToManyStruct<A, Edge, B> {

}