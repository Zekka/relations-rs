use experimental::*;
use experimental::onetoone::*;
use tests::utils::assert_iter_equal;

#[test]
fn selfish() {
    let mut human = Table::<&'static str>::new();

    let abraham = human.store("Abraham");
    let homer = human.store("Homer");
    let bart = human.store("Bart");
    let marge = human.store("Marge");

    let mut father = human.one_to_one_self(); // note: fatherhood is properly a one-to-many relation, but for this example we should be OK
    let mut mother = human.one_to_one_self(); // note: motherhood is properly a one-to-many relation, but for this example we should be OK

    { father.reify_mut(&mut human).map(homer, bart) }
    { father.reify_mut(&mut human).map(abraham, homer) }
    { mother.reify_mut(&mut human).map(marge, bart) }

    {
        let qfather = father.reify(&human);
        let qmother = mother.reify(&human);

        assert_eq!(qfather.to(bart), Some(homer));
        assert_eq!(qfather.to(homer), Some(abraham));
        assert_eq!(qfather.to(abraham), None);

        assert_eq!(qfather.from(bart), None);
        assert_eq!(qfather.from(homer), Some(bart));
        assert_eq!(qfather.from(abraham), Some(homer));

        assert_eq!(qmother.to(bart), Some(marge));
        assert_eq!(qmother.to(marge), None);

        assert_eq!(qmother.from(bart), None);
        assert_eq!(qmother.from(marge), Some(bart));
    }

    { human.free(homer); }

    {
        let qfather = father.reify(&human);
        let qmother = mother.reify(&human);

        assert_eq!(qfather.to(bart), None);
        assert_eq!(qfather.to(abraham), None);

        assert_eq!(qfather.from(bart), None);
        assert_eq!(qfather.from(abraham), None);

        assert_eq!(qmother.to(bart), Some(marge));
        assert_eq!(qmother.to(marge), None);

        assert_eq!(qmother.from(bart), None);
        assert_eq!(qmother.from(marge), Some(bart));
    }

    {
        // might as well test mutable version too!
        let mut qfather = father.reify_mut(&mut human);

        assert_eq!(qfather.to(bart), None);
        assert_eq!(qfather.to(abraham), None);

        assert_eq!(qfather.from(bart), None);
        assert_eq!(qfather.from(abraham), None);

        qfather.gc();

        assert_eq!(qfather.to(bart), None);
        assert_eq!(qfather.to(abraham), None);

        assert_eq!(qfather.from(bart), None);
        assert_eq!(qfather.from(abraham), None);
    }

    {
        // test mutate in-place
        mother.mutate_to(&mut human, Box::new(vec!(bart).into_iter()), |_, bartsmom| { 
            *bartsmom = "Selma";
        })
    }

    {
        let mut qmother = mother.reify(&human);

        assert_eq!(qmother.to(bart), Some(marge));
        assert_eq!(human.seek(marge), Some(&"Selma"));
    }
}