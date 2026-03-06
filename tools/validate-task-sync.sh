#!/bin/bash

echo "检查任务状态同步..."

total_changes=0
total_tasks=0
completed_tasks=0

# 遍历所有活跃 changes
for change_dir in openspec/changes/*/; do
    if [[ "$change_dir" == *"/archive/"* ]]; then
        continue
    fi
    
    if [[ ! -f "$change_dir/tasks.md" ]]; then
        continue
    fi
    
    change_name=$(basename "$change_dir")
    total_changes=$((total_changes + 1))
    
    # 统计任务
    task_count=$(grep -c "^- \[" "$change_dir/tasks.md" 2>/dev/null)
    completed_count=$(grep -c "^- \[x\]" "$change_dir/tasks.md" 2>/dev/null)
    
    # 如果 grep 失败，设置为 0
    task_count=${task_count:-0}
    completed_count=${completed_count:-0}
    
    total_tasks=$((total_tasks + task_count))
    completed_tasks=$((completed_tasks + completed_count))
    
    if [[ $task_count -gt 0 ]]; then
        pending=$((task_count - completed_count))
        echo "   [$change_name] 任务进度: $completed_count/$task_count 完成"
        
        if [[ $pending -gt 0 ]]; then
            echo "      未完成任务: $pending"
        fi
    fi
done

echo ""
echo "总计: $total_changes 个活跃 change, $completed_tasks/$total_tasks 任务完成"

if [[ $total_tasks -eq $completed_tasks ]]; then
    echo "[通过] 所有任务已完成"
else
    echo "[提示] 还有 $((total_tasks - completed_tasks)) 个任务未完成"
fi
