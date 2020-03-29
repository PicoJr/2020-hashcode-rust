use std::convert::TryInto;

pub type Book = usize;

#[derive(Clone)]
pub struct Library {
    pub id: usize,
    pub n_books: usize,
    pub signup_delay: usize,
    pub books_per_day: usize,
    pub books: Vec<Book>,
}

pub struct InputDataSet {
    pub n_books: usize,
    pub n_libraries: usize,
    pub n_days: usize,
    pub book_scores: Vec<usize>,
    pub libraries: Vec<Library>,
}

pub struct LibraryOrder {
    pub id: usize,
    pub n_books: usize,
    pub books: Vec<Book>,
}

pub struct OutputDataSet {
    pub n_libraries: usize,
    pub library_orders: Vec<LibraryOrder>,
}

fn n_best_books<F>(n: usize, books: &[Book], score: F) -> Vec<Book>
where
    F: Fn(Book) -> usize,
{
    let sorted_books: &mut [Book] = &mut books.to_vec();
    sorted_books.sort_unstable_by_key(|&book| -(score(book) as i64));
    sorted_books.to_vec().truncate(n);
    Vec::from(sorted_books)
}

fn max_books_sent(library: &Library, days_left: usize) -> i64 {
    let days_left: i64 = days_left as i64;
    let signup_delay: i64 = library.signup_delay as i64;
    let books_per_day: i64 = library.books_per_day as i64;
    (days_left - signup_delay) * books_per_day
}

fn library_best_books<F>(library: &Library, days_left: usize, score: F) -> Vec<Book>
where
    F: Fn(Book) -> usize,
{
    let max_books_sent_n = max_books_sent(library, days_left);
    if max_books_sent_n <= 0 {
        vec![]
    } else {
        // max_books_sent_n > 0, safe to cast to usize
        n_best_books(max_books_sent_n.try_into().unwrap(), &library.books, score)
    }
}

pub fn solve(input_data: &InputDataSet) -> OutputDataSet {
    let mut libraries_not_selected = input_data.libraries.to_vec();
    let mut updated_book_score = input_data.book_scores.to_vec();
    let mut days_left = input_data.n_days;

    fn best_library<'a>(
        libraries: &'a [Library],
        days_left: usize,
        updated_book_score: &[usize],
    ) -> Option<(usize, &'a Library)> {
        let indexed_libraries = libraries.iter().enumerate();
        indexed_libraries.max_by_key(|ilib| {
            let (_i, lib) = ilib;
            let best_books = library_best_books(lib, days_left, |b: Book| updated_book_score[b]);
            let heuristic: usize = best_books
                .iter()
                .map(|&b| updated_book_score[b])
                .sum::<usize>()
                / lib.signup_delay;
            heuristic
        })
    }
    let mut library_orders: Vec<LibraryOrder> = vec![];
    while !libraries_not_selected.is_empty() {
        let temp_libraries_not_selected = libraries_not_selected.clone();
        let (i_best, best) =
            best_library(&temp_libraries_not_selected, days_left, &updated_book_score)
                .expect("should not be empty");
        libraries_not_selected.remove(i_best);
        days_left -= best.signup_delay;
        let best_books = library_best_books(best, days_left, |b| updated_book_score[b]);
        let library_score: usize = best_books.iter().map(|&b| updated_book_score[b]).sum();
        if library_score == 0 {
            break; // best library score <= 0, useless to continue
        }
        for &book in &best_books {
            updated_book_score[book] = 0;
        }
        library_orders.push(LibraryOrder {
            id: best.id,
            n_books: best_books.len(),
            books: best_books.clone(),
        })
    }
    OutputDataSet {
        n_libraries: library_orders.len(),
        library_orders,
    }
}
