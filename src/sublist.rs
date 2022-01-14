/*
Given two lists determine if the first list is contained within the second list, if the second
list is contained within the first list, if both lists are contained within each other or
if none of these are true.

Specifically, a list A is a sublist of list B if by dropping 0 or more elements from the
front of B and 0 or more elements from the back of B you get a list that's completely equal to A.
Examples:
    A = [1, 2, 3], B = [1, 2, 3, 4, 5], A is a sublist of B
    A = [3, 4, 5], B = [1, 2, 3, 4, 5], A is a sublist of B
 */

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// My solution
/*
    Нахожу среди листов с большим и меньшим размером, далее ищу первый элемент
    меньшего листа среди элементов большего. Для каждого такого найденного элемента
    сравниваем следующие элементы с элементами меньшего листа.
 */
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == 0 && _second_list.len() != 0 {
        return Comparison::Sublist;
    }
    if _first_list.len() != 0 && _second_list.len() == 0 {
        return Comparison::Superlist;
    }

    let big = if _first_list.len() > _second_list.len() {
        _first_list
    } else {
        _second_list
    };
    let small = if _first_list.len() > _second_list.len() {
        _second_list
    } else {
        _first_list
    };

    if big == small {
        return Comparison::Equal;
    }

    let mut small_in_big = false;
    for i in 0..big.len() {
        if big[i] == small[0] {
            let mut j = 0;
            while j < small.len() && i + j < big.len() {
                if big[i + j] == small[j] {
                    j += 1;
                    continue;
                } else {
                    break;
                }
            }
            if j == small.len() {
                small_in_big = true;
                break;
            }
        }
    }

    if small_in_big {
        return if small == _first_list {
            Comparison::Sublist
        } else {
            Comparison::Superlist
        }
    }

    Comparison::Unequal
}

// Community
/*
    метод windows(n) возвращает итераторы на окна листа размером n,
    затем, если для какого-то окна нашли совпадение, выводим соответствующее решение.
 */
pub fn sublist_<T: Eq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => if a.windows(n).any(|v| v == b) {Superlist} else {Unequal},
        (m, n) if m < n => if b.windows(m).any(|v| v == a) {Sublist} else {Unequal},
        (_, _) => if a == b {Equal} else {Unequal},
    }
}