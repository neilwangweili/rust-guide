# 火星车

## Iteration 1

### Preview

假想你在火星探索团队中负责软件开发。现在你要给登陆火星的探索小车编写控制程序，根据地球发送的控制指令来控制火星车的行动。 火星车收到的指令分为四类：

- 探索区域信息：告知火星车，整片区域的长度（X）和宽度（Y）有多大；
- 初始化信息：火星车的降落地点（x, y）和朝向（N, S, E, W）信息；
- 移动指令：火星车可以前进（F）或后退（B）；
- 转向指令：火星车可以左转90度（L）或右转90度（R）。

由于地球和火星之间的距离很远，指令必须批量发送，火星车执行完整批指令之后，再回报自己所在的位置坐标和朝向。

### Scenarios

#### 基础概念：

1. MarsRover - 火星车
2. Area - 区域
3. Location - 位置
4. Direction - 方向
5. ExecutingCommands - 指令集
6. Command - 指令（特性）
    1. Turn - 转向（特性）
        1. TurnLeft - 左转
        2. TurnRight - 右转
    2. Run - 移动（特性）
        1. RunToward - 前进
        2. RunBack - 后退

#### 关系：

1 has a 2,3. 3 has a 4. 5 have some 6. Implementations of 6: Turn, Move.

### Tasking:

#### Unit Tests:

**Can init a Mars rover.**

- [x] should_init_a_mars_rover_with_x_max_10_y_max_15_x_3_y_3() -> "I'm 3 on the X-axis and 3 on the Y-axis and facing
  North."

**Can turn left.**

- [x] should_mars_rover_turn_left_to_east_at_north -> "I'm 3 on the X-axis and 3 on the Y-axis and facing East."
- [x] should_mars_rover_turn_left_to_south_at_east -> "I'm 3 on the X-axis and 3 on the Y-axis and facing South."
- [x] should_mars_rover_turn_left_to_west_at_south -> "I'm 3 on the X-axis and 3 on the Y-axis and facing West."
- [x] should_mars_rover_turn_left_to_north_at_west -> "I'm 3 on the X-axis and 3 on the Y-axis and facing North."

**Can turn right.**

- [x] should_mars_rover_turn_right_to_west_at_north -> "I'm 3 on the X-axis and 3 on the Y-axis and facing West."
- [x] should_mars_rover_turn_right_to_south_at_west -> "I'm 3 on the X-axis and 3 on the Y-axis and facing South."
- [x] should_mars_rover_turn_right_to_east_at_south -> "I'm 3 on the X-axis and 3 on the Y-axis and facing East."
- [x] should_mars_rover_turn_right_to_north_at_east -> "I'm 3 on the X-axis and 3 on the Y-axis and facing North."

**Can move toward.**

- [x] should_mars_rover_move_toward_at_east -> "I'm 4 on the X-axis and 3 on the Y-axis and facing East."
- [x] should_mars_rover_move_toward_at_south -> "I'm 3 on the X-axis and 4 on the Y-axis and facing South."
- [x] should_mars_rover_move_toward_at_west -> "I'm 2 on the X-axis and 3 on the Y-axis and facing West."
- [x] should_mars_rover_move_toward_at_north -> "I'm 3 on the X-axis and 2 on the Y-axis and facing North."

**Can move back.**

- [x] should_mars_rover_move_back_at_east -> "I'm 2 on the X-axis and 3 on the Y-axis and facing East."
- [x] should_mars_rover_move_back_at_south -> "I'm 3 on the X-axis and 2 on the Y-axis and facing South."
- [x] should_mars_rover_move_back_at_west -> "I'm 4 on the X-axis and 3 on the Y-axis and facing West."
- [x] should_mars_rover_move_back_at_north -> "I'm 3 on the X-axis and 4 on the Y-axis and facing North."

#### Integration tests:

- [x] x_max:10, y_max:15, x:3, y:3, direction: North. 前进,前进,左转,前进,右转,后退,左转,左转,后退: "I'm 4 on the X-axis and 1 on the
  Y-axis and facing East."

## Iteration 2

### Preview

下面是火星探索团队提出的其他一些需求：

- 应该阻止小车跑到超出整片区域坐标系的地方去
- 火星不是平的，而是球体，所以如果小车从坐标系的一边跑了出去，应该从另一边跑进来
- 火星地面上有障碍物，如果火星车遇到了障碍物导致后续指令受阻，应该停留在原地，放弃执行后续指令，并立即向地球回报
-

你需要自己判断，是否采纳这些需求。
