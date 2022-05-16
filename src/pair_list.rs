// Copied from https://github.com/ljedrz/blc/blob/6a673cf1b3b1a689e799912ab2b9ef852b810c04/src/pair_list.rs

use self::ListError::*;
use lambda_calculus::data::boolean::fls;
use lambda_calculus::*;

#[derive(Debug, PartialEq)]
pub enum ListError {
    NotAList,
}

pub fn uncons(term: Term) -> Result<(Term, Term), ListError> {
    if !is_list(&term) {
        Err(NotAList)
    } else {
        let candidate = if let Abs(abstracted) = term {
            *abstracted
        } else {
            term
        };

        if let Ok((wrapped_a, b)) = candidate.unapp() {
            if wrapped_a.rhs_ref().is_err() {
                Err(NotAList)
            } else {
                Ok((wrapped_a.rhs().unwrap(), b))
            }
        } else {
            Err(NotAList)
        }
    }
}

pub fn uncons_ref(term: &Term) -> Result<(&Term, &Term), ListError> {
    if !is_list(term) {
        return Err(NotAList);
    }
    let candidate = if let Abs(ref abstracted) = *term {
        abstracted
    } else {
        term
    };

    if let Ok((wrapped_a, b)) = candidate.unapp_ref() {
        if wrapped_a.rhs_ref().is_err() {
            Err(NotAList)
        } else {
            Ok((wrapped_a.rhs_ref().unwrap(), b))
        }
    } else {
        Err(NotAList)
    }
}

pub fn unpair_ref(term: &Term) -> Result<(&Term, &Term), ListError> {
    let candidate = if let Abs(ref abstracted) = *term {
        abstracted
    } else {
        term
    };

    if let Ok((wrapped_a, b)) = candidate.unapp_ref() {
        if wrapped_a.rhs_ref().is_err() {
            Err(NotAList)
        } else {
            Ok((wrapped_a.rhs_ref().unwrap(), b))
        }
    } else {
        Err(NotAList)
    }
}

pub fn is_pair(term: &Term) -> bool {
    unpair_ref(term).is_ok()
}

pub fn snd_ref(term: &Term) -> Result<&Term, ListError> {
    Ok(unpair_ref(term)?.1)
}

pub fn last_ref(term: &Term) -> Result<&Term, ListError> {
    if !is_pair(term) {
        return Err(NotAList);
    }

    let mut last_candidate = snd_ref(term)?;

    while let Ok(second) = snd_ref(last_candidate) {
        last_candidate = second;
    }

    Ok(last_candidate)
}

pub fn is_list(term: &Term) -> bool {
    last_ref(term) == Ok(&fls())
}

pub fn head_ref(term: &Term) -> Result<&Term, ListError> {
    Ok(uncons_ref(term)?.0)
}

pub fn tail(term: Term) -> Result<Term, ListError> {
    Ok(uncons(term)?.1)
}

pub fn push(list: Term, term: Term) -> Result<Term, ListError> {
    if !is_list(&list) && list != fls() {
        return Err(NotAList);
    }

    Ok(abs(app!(Var(1), term, list)))
}

pub fn listify_terms(terms: Vec<Term>) -> Term {
    let mut ret = fls();

    for term in terms.into_iter().rev() {
        ret = push(ret, term).expect("unwrap 4"); // safe - built from nil()
    }

    ret
}
