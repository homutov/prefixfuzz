from prefixfuzz import from_internal_data, PrefixSearch
from typing import List, Tuple


def from_nodes(data: List[Tuple[str, int]]) -> PrefixSearch:
    indexed_data = [(item[0], item[1], index) for index, item in enumerate(data)]

    # Add missed interim items with empty payloads
    prev_key = ""
    sorted_data = []
    for key, payload, orig_index in sorted(indexed_data, key=lambda item: item[0]):
        common_pos = 0
        while (common_pos < min(len(key) - 1, len(prev_key))) and (key[common_pos] == prev_key[common_pos]):
            common_pos += 1

        for key_pos in range(common_pos + 1, len(key)):
            sorted_data.append((key[:key_pos], None, 1_000_000_000))

        sorted_data.append((key, payload, orig_index))
        prev_key = key

    if len(sorted_data[0][0]) > 0:
        # Add root node with empty payload
        sorted_data = [("", None, 1_000_000_000)] + sorted_data

    # Assign Node IDs
    data_with_ids = [
        (node_id, key, payload, orig_index) for node_id, (key, payload, orig_index) in enumerate(sorted_data)
    ]

    max_len = max([len(item[0]) for item in sorted_data])
    children_acc = [[] for _ in range(max_len)]
    children_lists = []

    prev_len = -1
    for node_id, key, payload, orig_index in reversed(data_with_ids):
        if len(key) < prev_len:
            node_children = list((ch[1], ch[0]) for ch in sorted(children_acc[prev_len - 1], key=lambda ch: ch[2]))
            children_lists.append(node_children)
            children_acc[prev_len - 1] = []
        else:
            children_lists.append([])

        if len(key) > 0:
            children_acc[len(key) - 1].append((node_id, key[-1], orig_index))

        prev_len = len(key)

    children_lists = list(reversed(children_lists))

    node_shifts = []
    child_transitions = []
    child_labels = [None] * len(data_with_ids)

    for node_children in children_lists:
        node_shifts.append(len(child_transitions))
        for child_label, child_transition in node_children:
            # TODO: counter-intuitive usage of child labels
            child_labels[child_transition] = child_label
            child_transitions.append(child_transition)

    payloads = [payload for _, _, payload, _ in data_with_ids]

    # Build prefix search
    child_labels[0] = ""

    return from_internal_data(node_shifts, child_labels, payloads, child_transitions)
