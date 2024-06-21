use rutils::max;

use crate::hierarchy::{ConcreteSelector, NodeHierarchyComponent};

pub fn concrete_selector_groups_do_match(hierarchy: &[NodeHierarchyComponent], concrete_selector_groups: &[Vec<ConcreteSelector<'_>>]) -> bool {
	let mut concrete_selector_group_index = 0;
	let mut hierarchy_indexes = None;

	loop {
		let selector_group = match concrete_selector_groups.get(concrete_selector_group_index) {
			Some(group) => group,
			None => break,
		};

		let mut new_hierarchy_indexes = Vec::new();

		if let Some(old_hierarchy_indexes) = hierarchy_indexes {
			for index in old_hierarchy_indexes {
				let new_indexes = get_all_concrete_selector_match_indexes(&hierarchy[index..], &selector_group);

				for new_index in new_indexes {
					new_hierarchy_indexes.push(index + new_index);
				}
			}
		} else {
			new_hierarchy_indexes.append(&mut get_all_concrete_selector_match_indexes(hierarchy, &selector_group))
		}

		// If there are selectors that we can't satisify with hierarchy components, this is certainly not a match
		if new_hierarchy_indexes.is_empty() {
			return false;
		}

		hierarchy_indexes = Some(new_hierarchy_indexes);
		concrete_selector_group_index += 1;
	}

	let hierarchy_indexes = match hierarchy_indexes {
		Some(indexes) => indexes,
		None => return false, // not exactly sure how we would get here, but it doesn't look like a match
	};

	// Now, we just have to make sure that the selectors walked all the way to the last hierarchy component
	// we can unwrap here because max only returns None if it's array is empty, but we can ensure it is not
	let greatest_hierarchy_index = max(&hierarchy_indexes).unwrap();

	greatest_hierarchy_index == hierarchy.len()
}

fn get_all_concrete_selector_match_indexes(hierarchy: &[NodeHierarchyComponent], concrete_selectors: &[ConcreteSelector<'_>]) -> Vec<usize> {
	let mut indexes = Vec::new();

	loop {
		let last_index = indexes.last().map(|u| *u).unwrap_or(0);

		match get_concrete_selectors_match_index(&hierarchy[last_index..], concrete_selectors) {
			Some(index) => indexes.push(index),
			None => break,
		}
	}

	indexes
}

fn get_concrete_selectors_match_index(hierarchy: &[NodeHierarchyComponent], concrete_selectors: &[ConcreteSelector<'_>]) -> Option<usize> {
	let mut hierarchy_index_before_selection_run = 0;
	let mut hierarchies_checked_in_current_selection_run = 0;
	let mut selector_index = 0;

	loop {
		let current_hierarchy_index = hierarchy_index_before_selection_run + hierarchies_checked_in_current_selection_run;

		// the only time that we would have found a match is if we reached the end of our concrete selectors at or before reaching the end
		// of our hierararchy components. This statement must appear before we check for a hierarchy because it is still considered a match
		// when the selector and component reach the end of themselves at the same time
		let selector = match concrete_selectors.get(selector_index) {
			Some(selector) => selector,
			None => {
				// We don't want to respond with an index that is in the middle of an element, because that would make any retries inefficent
				let component_count_till_next_boundary = get_next_element_boundary_index(&hierarchy[current_hierarchy_index..]);

				return Some(current_hierarchy_index + component_count_till_next_boundary);
			}
		};
		let component = match hierarchy.get(current_hierarchy_index) {
			Some(component) => component,
			None => return None,
		};

		let match_resolution = hierarchy_component_matches_concrete_selector(component, selector);

		match match_resolution {
			MatchResolution::Success => {
				selector_index += 1;
				hierarchies_checked_in_current_selection_run += 1;
			}
			MatchResolution::Skip => {
				hierarchies_checked_in_current_selection_run += 1;
			}
			// restart the selection at the next element in the hierarchy from where we started the selection.
			// Note: we might not be passing the entire selection attempt if it spanned multiple element boundaries
			MatchResolution::Failure => {
				selector_index = 0;
				hierarchies_checked_in_current_selection_run = 0;

				let component_count_till_next_boundary = get_next_element_boundary_index(&hierarchy[hierarchy_index_before_selection_run..]);
				hierarchy_index_before_selection_run += component_count_till_next_boundary;
			}
		}
	}
}

fn get_next_element_boundary_index(hierarchy: &[NodeHierarchyComponent]) -> usize {
	let mut index = 0;

	loop {
		let component = match hierarchy.get(index) {
			Some(component) => component,
			None => return hierarchy.len(),
		};

		index += 1;

		if let NodeHierarchyComponent::Child = component {
			break;
		}
	}

	index
}

enum MatchResolution {
	/// Whatever we're trying to match just matched
	Success,
	/// Whatever we just tried to match didn't match, but it might match the next time this function is called (with incremented arguments,
	/// normally).
	Skip,
	/// Whatever we just tried to match has failed and won't ever be able to succeed on a future call to this function.
	Failure,
}

fn hierarchy_component_matches_concrete_selector(component: &NodeHierarchyComponent, selector: &ConcreteSelector<'_>) -> MatchResolution {
	match component {
		// a failure to match a root is always a failure. There is not another root that it could match later
		NodeHierarchyComponent::Root => match selector {
			ConcreteSelector::Root => MatchResolution::Success,
			_ => MatchResolution::Failure,
		},
		// a failure to match a tag is also always a failure. While another tag might be matched later, it will be in a complete retry of the
		// matching operation, not a simple skip to the next component
		NodeHierarchyComponent::Tag(name) => match selector {
			ConcreteSelector::Tag(selected_name) => {
				if name == selected_name {
					MatchResolution::Success
				} else {
					MatchResolution::Failure
				}
			}
			// HACK: this really needs to be cleaned out. Works well when the only element query constraints are tag and class, but will blow up into an evil
			// montrosity if ids and other attributes are ever added.
			ConcreteSelector::Class(_) => MatchResolution::Skip,
			_ => MatchResolution::Failure,
		},
		// a failure to match a class name might not be a real failure. Multiple classnames can appear for every element, so we might be able
		// to match the next one
		NodeHierarchyComponent::Class(name) => match selector {
			ConcreteSelector::Class(selected_name) => {
				if name == selected_name {
					MatchResolution::Success
				} else {
					MatchResolution::Skip
				}
			}
			_ => MatchResolution::Skip,
		},
		// a failure to match a child (i.e. the end of an element) is always a real failure. If the selector is expecting a tag name or class name, it should've
		// already been matched
		NodeHierarchyComponent::Child => match selector {
			ConcreteSelector::Child => MatchResolution::Success,
			_ => MatchResolution::Failure,
		},
	}
}
