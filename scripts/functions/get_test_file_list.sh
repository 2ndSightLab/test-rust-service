#!/bin/bash

get_test_file_list() {
    # Get top-level local test files that cargo can execute
    local local_test_dir=$(get_test_dir "local")
    if [[ -d "$local_test_dir" ]]; then
        find "$local_test_dir" -maxdepth 1 -name "*.rs" -exec grep -l "pub mod test_" {} \; 2>/dev/null
    fi
    
    # Get top-level common test files that cargo can execute
    local common_test_dir=$(get_test_dir "common" 2>/dev/null)
    if [[ -n "$common_test_dir" && -d "$common_test_dir" ]]; then
        find "$common_test_dir" -maxdepth 1 -name "*.rs" -exec grep -l "pub mod test_" {} \; 2>/dev/null
    fi
}
