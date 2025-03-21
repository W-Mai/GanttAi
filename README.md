# GanttA! (干它！)

# 功能

1. 自动分析输入的内容

    1. 总结出简短概要
    2. 将概要作为标题
    3. 原始内容作为原始内容
    4. 分析任务起始和结束时间点

        1. 内容中包含未来时间点，将未来时间点作为提醒点

            1. “10min 提醒我一次”：设置通知，每 10min 提醒一次 等
            2. “9点半”提醒我：设置通知，9点半提醒一次
            3. “明天下午三点提醒我”：设置的通知，明天下午三点提醒一次
        2. 如果可以通过经验分析出该任务的可能起始和结束时间点，那么按照分析出的结果让用户确认是否为该时间点
        3. 否则当前的时间点作为任务起始时间点
    5. 如果输入内容和目前的任务匹配

        1. 告知增加新的提醒：弹出提示，询问是否味该任务，其余同 1.4.a
        2. 告知任务结束，那就弹出提示，询问是否是结束某个任务
        3. 如果分析出某任务为其他任务的子任务，则将其添加为子任务，并合理调整父任务的时间节点
    6. 搜索

        1. 根据用户提示词确定当前是否为搜索功能
2. 绘制当前所有任务的甘特图

    1. 起始时间
    2. 结束时间，如果没有设置结束时间，那么永远都是现在的时间
    3. 显示任务大纲
3. 总结

    1. 日度总结
    2. 周度总结
    3. 月度总结
    4. 季度总结
    5. 年度总结
    6. 从使用该软件到现在的总结
    7. 人生总结
