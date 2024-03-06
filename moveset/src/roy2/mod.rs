use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

static mut originY : f32 = 0.0;
static mut originZ : f32 = 0.0;

static mut specialNAlive: bool = false;
#[acmd_script( agent = "roy", script = "game_specialnstart", category = ACMD_GAME, low_priority )]
unsafe fn roy_specialn(agent: &mut L2CAgentBase) {
    if (!specialNAlive) {
        originY = PostureModule::pos_y(agent.module_accessor);
        originZ = PostureModule::pos_z(agent.module_accessor);
    }    
        for n in 1..70 {
            frame(agent.lua_state_agent, n as f32);
            if (n > 10) {
                if macros::is_excute(agent) {
                    specialNAlive = true;
                    macros::ATTACK(
                        agent,
                        4,
                        0,
                        Hash40::new("top"),
                        8.0,
                        55,
                        60,
                        0,
                        38,
                        10.0,
                        6.0,
                        6.0 + (originY - PostureModule::pos_y(agent.module_accessor)),
                        ((n * 2) - 10) as f32
                            + (originZ - PostureModule::pos_z(agent.module_accessor)),
                        None,
                        None,
                        None,
                        0.5,
                        1.0,
                        *ATTACK_SETOFF_KIND_ON,
                        *ATTACK_LR_CHECK_SPEED,
                        false,
                        -4,
                        0.0,
                        0,
                        true,
                        true,
                        false,
                        true,
                        false,
                        *COLLISION_SITUATION_MASK_GA,
                        *COLLISION_CATEGORY_MASK_NO_FLOOR,
                        *COLLISION_PART_MASK_ALL,
                        false,
                        Hash40::new("collision_attr_cutup"),
                        *ATTACK_SOUND_LEVEL_M,
                        *COLLISION_SOUND_ATTR_CUTUP,
                        *ATTACK_REGION_NONE,
                    );
                }
            }
        }
    

    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        specialNAlive = false;
        AttackModule::clear_all(agent.module_accessor);
        StatusModule::change_status_request_from_script(
            agent.module_accessor,
            FIGHTER_STATUS_KIND_WAIT.into(),
            false.into(),
        );
    }
}

#[acmd_script( agent = "roy", script = "game_specialnloop", category = ACMD_GAME, low_priority )]
unsafe fn roy_specialnloop(agent: &mut L2CAgentBase) {}
#[acmd_script( agent = "roy", script = "game_specialnend", category = ACMD_GAME, low_priority )]
unsafe fn roy_specialnend(agent: &mut L2CAgentBase) {}

#[acmd_script( agent = "roy", script = "effect_specialnstart", category = ACMD_EFFECT, low_priority )]
unsafe fn roy_specialn_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if (!specialNAlive) {
        originY = PostureModule::pos_y(agent.module_accessor);
        originZ = PostureModule::pos_z(agent.module_accessor);
    } 

    for n in 1..70 {
        frame(agent.lua_state_agent, n as f32);
        if (n > 10) {
            if macros::is_excute(agent) {
                macros::EFFECT(
                    agent,
                    Hash40::new("roy_mc_1"),
                    Hash40::new("top"),
                    0,
                    0.0 + (originY - PostureModule::pos_y(agent.module_accessor)),
                    ((n * 2) - 18) as f32
                        + (originZ - PostureModule::pos_z(agent.module_accessor)),
                    0,
                    0,
                    0,
                    0.7,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    true,
                );
            }
        }
    }
}

#[acmd_script( agent = "roy", script = "game_specialairnstart", category = ACMD_GAME, low_priority )]
unsafe fn roy_specialairn(agent: &mut L2CAgentBase) {
    if (!specialNAlive) {
        originY = PostureModule::pos_y(agent.module_accessor);
        originZ = PostureModule::pos_z(agent.module_accessor);
    } 
    for n in 1..70 {
        frame(agent.lua_state_agent, n as f32);
        if (n > 10) {
            if macros::is_excute(agent) {
                specialNAlive = true;
                macros::ATTACK(
                    agent,
                    4,
                    0,
                    Hash40::new("top"),
                    8.0,
                    55,
                    60,
                    0,
                    38,
                    10.0,
                    6.0,
                    6.0 + (originY - PostureModule::pos_y(agent.module_accessor)),
                    ((n * 2) - 10) as f32
                        + (originZ - PostureModule::pos_z(agent.module_accessor)),
                    None,
                    None,
                    None,
                    0.5,
                    1.0,
                    *ATTACK_SETOFF_KIND_ON,
                    *ATTACK_LR_CHECK_SPEED,
                    false,
                    -4,
                    0.0,
                    0,
                    true,
                    true,
                    false,
                    true,
                    false,
                    *COLLISION_SITUATION_MASK_GA,
                    *COLLISION_CATEGORY_MASK_NO_FLOOR,
                    *COLLISION_PART_MASK_ALL,
                    false,
                    Hash40::new("collision_attr_cutup"),
                    *ATTACK_SOUND_LEVEL_M,
                    *COLLISION_SOUND_ATTR_CUTUP,
                    *ATTACK_REGION_NONE,
                );
            }
        }
    }

    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        specialNAlive = false;
        AttackModule::clear_all(agent.module_accessor);
        StatusModule::change_status_request_from_script(
            agent.module_accessor,
            FIGHTER_STATUS_KIND_WAIT.into(),
            false.into(),
        );
    }
}

#[acmd_script( agent = "roy", script = "game_specialairnloop", category = ACMD_GAME, low_priority )]
unsafe fn roy_specialairnloop(agent: &mut L2CAgentBase) {}
#[acmd_script( agent = "roy", script = "game_specialairnend", category = ACMD_GAME, low_priority )]
unsafe fn roy_specialairnend(agent: &mut L2CAgentBase) {}

#[acmd_script( agent = "roy", script = "effect_specialairnstart", category = ACMD_EFFECT, low_priority )]
unsafe fn roy_specialairn_fx(agent: &mut L2CAgentBase) {
    if (!specialNAlive) {
        originY = PostureModule::pos_y(agent.module_accessor);
        originZ = PostureModule::pos_z(agent.module_accessor);
    } 
    for n in 1..70 {
        frame(agent.lua_state_agent, n as f32);
        if (n > 10) {
            if macros::is_excute(agent) {
                macros::EFFECT(
                    agent,
                    Hash40::new("roy_mc_1"),
                    Hash40::new("top"),
                    0,
                    0.0 + (originY - PostureModule::pos_y(agent.module_accessor)),
                    ((n * 2) - 18) as f32
                        + (originZ - PostureModule::pos_z(agent.module_accessor)),
                    0,
                    0,
                    0,
                    0.7,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    true,
                );
            }
        }
    }
}
//==================================================================================================
#[acmd_script( agent = "roy", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn roy_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(
            agent,
            Hash40::new_raw(0x2127e37c07),
            *GROUND_CLIFF_CHECK_KIND_ALWAYS
        );
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        shield!(
            agent,
            *MA_MSC_CMD_REFLECTOR,
            *COLLISION_KIND_REFLECTOR,
            0,                  // ID
            Hash40::new("top"), // Bone
            7.5,                // Size
            0.0,
            6.7,
            13.0, // X1, Y1, Z1
            0.0,
            6.7,
            13.0,  // X2, Y2, Z2
            2.0,   // Damage multiplier
            1.5,   // Speed multiplier
            200,   // Max damage
            false, // Unknown
            0.5,   // Life multiplier
            *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT
        );
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            6.0,
            110,
            100,
            80,
            0,
            9.0,
            0.0,
            6.7,
            13.0,
            None,
            None,
            None,
            0.0,
            1.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_turn"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_CUTUP,
            *ATTACK_REGION_OBJECT,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("top"),
            6.0,
            110,
            100,
            80,
            0,
            6.0,
            0.0,
            6.7,
            5.0,
            None,
            None,
            None,
            0.0,
            1.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_turn"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_CUTUP,
            *ATTACK_REGION_OBJECT,
        );
    }

    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(
            agent,
            Hash40::new_raw(0x2127e37c07),
            *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES
        );
        AttackModule::clear_all(agent.module_accessor);
        ShieldModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "roy", script = "effect_speciallw", category = ACMD_EFFECT, low_priority )]
unsafe fn roy_speciallw_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("sys_damage_paralysis"),
            Hash40::new("top"),
            -3,
            10,
            14,
            0,
            0.0,
            0,
            0.8,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

#[acmd_script( agent = "roy", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn roy_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.3);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(
            agent,
            Hash40::new_raw(0x2127e37c07),
            *GROUND_CLIFF_CHECK_KIND_ALWAYS
        );
    }
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        shield!(
            agent,
            *MA_MSC_CMD_REFLECTOR,
            *COLLISION_KIND_REFLECTOR,
            0,                  // ID
            Hash40::new("top"), // Bone
            7.5,                // Size
            0.0,
            6.7,
            13.0, // X1, Y1, Z1
            0.0,
            6.7,
            13.0,  // X2, Y2, Z2
            2.0,   // Damage multiplier
            1.5,   // Speed multiplier
            200,   // Max damage
            false, // Unknown
            0.5,   // Life multiplier
            *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT
        );
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            6.0,
            110,
            100,
            80,
            0,
            9.0,
            0.0,
            6.7,
            13.0,
            None,
            None,
            None,
            0.0,
            1.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_turn"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_CUTUP,
            *ATTACK_REGION_OBJECT,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("top"),
            6.0,
            110,
            100,
            80,
            0,
            6.0,
            0.0,
            6.7,
            5.0,
            None,
            None,
            None,
            0.0,
            1.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_turn"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_CUTUP,
            *ATTACK_REGION_OBJECT,
        );
    }

    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(
            agent,
            Hash40::new_raw(0x2127e37c07),
            *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES
        );
        AttackModule::clear_all(agent.module_accessor);
        ShieldModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "roy", script = "effect_specialairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn roy_specialairlw_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("sys_damage_paralysis"),
            Hash40::new("top"),
            -3,
            10,
            14,
            0,
            0.0,
            0,
            0.8,
            true,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

//==========================================================================================================================

#[acmd_script( agent = "roy", script = "game_specials1", category = ACMD_GAME, low_priority )]
unsafe fn roy_specials1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.2);
    hasSpecialSEffectHappened = false;
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        if (PostureModule::lr(agent.module_accessor) == -1.0) {
            PostureModule::set_pos(
                agent.module_accessor,
                &Vector3f {
                    x: PostureModule::pos_x(agent.module_accessor) - 60.0,
                    y: PostureModule::pos_y(agent.module_accessor),
                    z: PostureModule::pos_z(agent.module_accessor),
                },
            );
        } else {
            PostureModule::set_pos(
                agent.module_accessor,
                &Vector3f {
                    x: PostureModule::pos_x(agent.module_accessor) + 60.0,
                    y: PostureModule::pos_y(agent.module_accessor),
                    z: PostureModule::pos_z(agent.module_accessor),
                },
            );
        }
        StatusModule::change_status_request_from_script(
            agent.module_accessor,
            FIGHTER_ROY_STATUS_KIND_SPECIAL_S4.into(),
            false.into(),
        );
    }
}

#[acmd_script( agent = "roy", script = "game_specials4s", category = ACMD_GAME, low_priority )]
unsafe fn roy_specialsend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);

    frame(agent.lua_state_agent, 14.0);
    if (PostureModule::lr(agent.module_accessor) == -1.0) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            7.0,
            93,
            50,
            0,
            80,
            3.0,
            0.0,
            5.0,
            0.0,
            Some(0.0),
            Some(5.0),
            Some(-60.0),
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
            *ATTACK_LR_CHECK_B,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_elec"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_BODY,
        );
    } else {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            7.0,
            93,
            50,
            0,
            80,
            3.0,
            0.0,
            5.0,
            0.0,
            Some(0.0),
            Some(5.0),
            Some(-60.0),
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
            *ATTACK_LR_CHECK_B,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_elec"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_BODY,
        );
    }
    frame(agent.lua_state_agent, 18.0);
    AttackModule::clear_all(agent.module_accessor);
}

#[acmd_script( agent = "roy", script = "effect_specials1", category = ACMD_EFFECT, low_priority )]
unsafe fn roy_specials1_fx(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_h_smoke_a"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            0,
            0,
            0.7,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.2, 0.9, 1, 0.4);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    wait(agent.lua_state_agent, 1.0);
}
static mut hasSpecialSEffectHappened: bool = false;
#[acmd_script( agent = "roy", script = "effect_specials4s", category = ACMD_EFFECT, low_priority )]
unsafe fn roy_specialsend_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if (!hasSpecialSEffectHappened) {
        for n in 0..5 {
            macros::EFFECT(
                agent,
                Hash40::new("sys_mball_beam"),
                Hash40::new("top"),
                0,
                5,
                ((-12.0) * n as f32),
                0,
                0,
                0,
                1.5,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
        }
    }
}

//==========================================================================================================================

#[acmd_script( agent = "roy", script = "game_specialairs1", category = ACMD_GAME, low_priority )]
unsafe fn roy_specialairs1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    KineticModule::unable_energy_all(agent.module_accessor);
    macros::FT_MOTION_RATE(agent, 0.68);
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        /// Returns the current direction the current battle object is facing. -1 = left, 1 = right
        if (PostureModule::lr(agent.module_accessor) == -1.0) {
            PostureModule::set_pos(
                agent.module_accessor,
                &Vector3f {
                    x: PostureModule::pos_x(agent.module_accessor) - 60.0,
                    y: PostureModule::pos_y(agent.module_accessor),
                    z: PostureModule::pos_z(agent.module_accessor),
                },
            );
        } else {
            PostureModule::set_pos(
                agent.module_accessor,
                &Vector3f {
                    x: PostureModule::pos_x(agent.module_accessor) + 60.0,
                    y: PostureModule::pos_y(agent.module_accessor),
                    z: PostureModule::pos_z(agent.module_accessor),
                },
            );
        }
        StatusModule::change_status_request_from_script(
            agent.module_accessor,
            FIGHTER_ROY_STATUS_KIND_SPECIAL_S4.into(),
            false.into(),
        );
    }
}

#[acmd_script( agent = "roy", script = "effect_specialairs1", category = ACMD_EFFECT, low_priority )]
unsafe fn roy_specialairs1_fx(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_h_smoke_a"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            0,
            0,
            0.7,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.2, 0.9, 1, 0.4);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        //macros::COL_NORMAL(agent);
    }
    wait(agent.lua_state_agent, 1.0);
}

#[acmd_script( agent = "roy", script = "game_specialairs4s", category = ACMD_GAME, low_priority )]
unsafe fn roy_specialairs4s(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    KineticModule::unable_energy(agent.module_accessor, 1);
    frame(agent.lua_state_agent, 14.0);
    if (PostureModule::lr(agent.module_accessor) == -1.0) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            7.0,
            93,
            50,
            0,
            80,
            3.0,
            0.0,
            5.0,
            0.0,
            Some(0.0),
            Some(5.0),
            Some(-60.0),
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
            *ATTACK_LR_CHECK_B,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_elec"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_BODY,
        );
    } else {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            7.0,
            93,
            50,
            0,
            80,
            3.0,
            0.0,
            5.0,
            0.0,
            Some(0.0),
            Some(5.0),
            Some(-60.0),
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
            *ATTACK_LR_CHECK_B,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_elec"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_ELEC,
            *ATTACK_REGION_BODY,
        );
    }
    frame(agent.lua_state_agent, 18.0);
    KineticModule::enable_energy(agent.module_accessor, 1);
    AttackModule::clear_all(agent.module_accessor);
}

#[acmd_script( agent = "roy", script = "effect_specialairs4s", category = ACMD_EFFECT, low_priority )]
unsafe fn roy_specialairs4s_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    for n in 0..5 {
        macros::EFFECT(
            agent,
            Hash40::new("sys_mball_beam"),
            Hash40::new("top"),
            0,
            5,
            ((-12.0) * n as f32),
            0,
            0,
            0,
            1.5,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
    hasSpecialSEffectHappened = true;
}

//==========================================================================================================================

#[acmd_script( agent = "roy", script = "game_final", category = ACMD_GAME, low_priority )]
unsafe fn roy_final(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            35.0,
            361,
            47,
            0,
            90,
            10.0,
            0.0,
            7.0,
            5.0,
            None,
            None,
            None,
            1.2,
            0.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            f32::NAN,
            0.0,
            0,
            false,
            false,
            false,
            false,
            false,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_fire"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_NONE,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("top"),
            35.0,
            361,
            47,
            0,
            90,
            16.0,
            0.0,
            12.5,
            17.5,
            None,
            None,
            None,
            1.2,
            0.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            f32::NAN,
            0.0,
            0,
            false,
            false,
            false,
            false,
            false,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_fire"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_NONE,
        );

        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(agent.module_accessor, 1, true, true, -1.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::CANCEL_FILL_SCREEN(agent, 0, 40);
    }
    for n in 30..90 {
        if (n > 30) {
            frame(agent.lua_state_agent, n as f32);

            if macros::is_excute(agent) {
                macros::ATTACK(
                    agent,
                    0,
                    0,
                    Hash40::new("top"),
                    35.0,
                    361,
                    47,
                    0,
                    90,
                    10.0,
                    0.0,
                    5.0,
                    ((n * 8) - 240) as f32,
                    Some(0.0),
                    Some(55.0),
                    Some(((n * 8) - 240) as f32),
                    1.2,
                    0.0,
                    *ATTACK_SETOFF_KIND_OFF,
                    *ATTACK_LR_CHECK_F,
                    false,
                    f32::NAN,
                    0.0,
                    0,
                    false,
                    false,
                    false,
                    false,
                    false,
                    *COLLISION_SITUATION_MASK_GA,
                    *COLLISION_CATEGORY_MASK_ALL,
                    *COLLISION_PART_MASK_ALL,
                    false,
                    Hash40::new("collision_attr_fire"),
                    *ATTACK_SOUND_LEVEL_L,
                    *COLLISION_SOUND_ATTR_FIRE,
                    *ATTACK_REGION_NONE,
                );
            }
        }
    }
}

#[acmd_script( agent = "roy", script = "effect_finalstart", category = ACMD_EFFECT, low_priority )]
unsafe fn roy_finalstart_fx(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {}
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_BRANCH_SITUATION(agent, Hash40::new("none"), Hash40::new("sys_shield_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("roy_sword"),
            Hash40::new("sword1"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            true,
        );
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("roy_final_sword"),
            Hash40::new("sword1"),
            0,
            0.03,
            0.3,
            89.5,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("roy_final_sword"), false, false);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_shield_smoke"), false, false);
    }
}

#[acmd_script( agent = "roy", script = "effect_final", category = ACMD_EFFECT, low_priority )]
unsafe fn roy_final_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_ROY_STATUS_FINAL_FLAG_REMOVE_FINAL_AURA,
        );
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("roy_sword"),
            Hash40::new("sword1"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, 0x0cca57558b, Hash40::new("top"), 0, 15, 6, 0, -30, -140, 1.7, false);
    }

    for n in 5..90 {
        wait(agent.lua_state_agent, 1.0);
        if (n > 30) {
            frame(agent.lua_state_agent, n as f32);

            if macros::is_excute(agent) {
                macros::EFFECT(
                    agent,
                    Hash40::new("roy_mc_1"),
                    Hash40::new("top"),
                    0,
                    5,
                    ((n * 8) - 240) as f32,
                    0,
                    0,
                    0,
                    2.5,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    true,
                );
                if (n > 78) {
                    if macros::is_excute(agent) {
                        macros::EFFECT_OFF_KIND(agent, Hash40::new("roy_sword"), false, false);
                        macros::EFFECT_OFF_KIND(
                            agent,
                            Hash40::new("roy_final_sword_fire"),
                            false,
                            false,
                        );
                    }
                }
            }
        }
    }

    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_h_smoke_b"),
            Hash40::new("top"),
            -7,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("roy_sword"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("roy_final_sword_fire"), false, false);
    }
}

#[acmd_script( agent = "roy", script = "game_finalair", category = ACMD_GAME, low_priority )]
unsafe fn roy_finalair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            35.0,
            361,
            47,
            0,
            90,
            10.0,
            0.0,
            7.0,
            5.0,
            None,
            None,
            None,
            1.2,
            0.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            f32::NAN,
            0.0,
            0,
            false,
            false,
            false,
            false,
            false,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_fire"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_NONE,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("top"),
            35.0,
            361,
            47,
            0,
            90,
            16.0,
            0.0,
            12.5,
            17.5,
            None,
            None,
            None,
            1.2,
            0.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            f32::NAN,
            0.0,
            0,
            false,
            false,
            false,
            false,
            false,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_fire"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_FIRE,
            *ATTACK_REGION_NONE,
        );
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_final_finish_cut_in(agent.module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_final_finish_cut_in(agent.module_accessor, 1, true, true, -1.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::CANCEL_FILL_SCREEN(agent, 0, 40);
    }
    for n in 30..90 {
        if (n > 30) {
            frame(agent.lua_state_agent, n as f32);
            if macros::is_excute(agent) {
                macros::ATTACK(
                    agent,
                    0,
                    0,
                    Hash40::new("top"),
                    35.0,
                    361,
                    47,
                    0,
                    90,
                    10.0,
                    0.0,
                    5.0,
                    ((n * 8) - 240) as f32,
                    Some(0.0),
                    Some(55.0),
                    Some(((n * 8) - 240) as f32),
                    1.2,
                    0.0,
                    *ATTACK_SETOFF_KIND_OFF,
                    *ATTACK_LR_CHECK_F,
                    false,
                    f32::NAN,
                    0.0,
                    0,
                    false,
                    false,
                    false,
                    false,
                    false,
                    *COLLISION_SITUATION_MASK_GA,
                    *COLLISION_CATEGORY_MASK_ALL,
                    *COLLISION_PART_MASK_ALL,
                    false,
                    Hash40::new("collision_attr_fire"),
                    *ATTACK_SOUND_LEVEL_L,
                    *COLLISION_SOUND_ATTR_FIRE,
                    *ATTACK_REGION_NONE,
                );
            }
        }
    }
    frame(agent.lua_state_agent, 40.0);
    macros::FT_MOTION_RATE(agent, 0.8);
}

#[acmd_script( agent = "roy", script = "effect_finalairstart", category = ACMD_EFFECT, low_priority )]
unsafe fn roy_finalairstart_fx(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::req_screen(
            agent.module_accessor,
            Hash40::new("bg_roy_final"),
            false,
            false,
            false,
        );
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_BRANCH_SITUATION(agent, Hash40::new("none"), Hash40::new("sys_shield_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("roy_sword"),
            Hash40::new("sword1"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            true,
        );
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("roy_final_sword"),
            Hash40::new("sword1"),
            0,
            0.03,
            0.3,
            89.5,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("roy_final_sword"), false, false);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("roy_final_light"),
            Hash40::new("sword1"),
            0,
            0,
            12,
            0,
            0,
            0,
            0.75,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
        macros::EFFECT(
            agent,
            Hash40::new("roy_final_lensflare"),
            Hash40::new("sword1"),
            0,
            0,
            0,
            0,
            180,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_shield_smoke"), false, false);
    }
}

#[acmd_script( agent = "roy", script = "effect_finalair", category = ACMD_EFFECT, low_priority )]
unsafe fn roy_finalair_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_ROY_STATUS_FINAL_FLAG_REMOVE_FINAL_AURA,
        );
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("roy_sword"),
            Hash40::new("sword1"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("roy_final_slash"),
            Hash40::new("top"),
            0,
            15,
            6,
            0,
            -30,
            -140,
            1.7,
            false,
        );
    }
    for n in 5..90 {
        wait(agent.lua_state_agent, 1.0);
        if (n > 30) {
            frame(agent.lua_state_agent, n as f32);

            if macros::is_excute(agent) {
                macros::EFFECT(
                    agent,
                    Hash40::new("roy_mc_1"),
                    Hash40::new("top"),
                    0,
                    5,
                    ((n * 8) - 240) as f32,
                    0,
                    0,
                    0,
                    2.5,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    true,
                );
                if (n > 78) {
                    if macros::is_excute(agent) {
                        macros::EFFECT_OFF_KIND(agent, Hash40::new("roy_sword"), false, false);
                        macros::EFFECT_OFF_KIND(
                            agent,
                            Hash40::new("roy_final_sword_fire"),
                            false,
                            false,
                        );
                    }
                }
            }
        }
    }
}
//==========================================================================================================================

//==============================================================================================================
//#[smashline::installer]
pub fn install() {
    smashline::install_acmd_scripts!(
        roy_speciallw,
        roy_specialairlw,
        roy_speciallw_fx,
        roy_specialairlw_fx,
        roy_specials1,
        roy_specials1_fx,
        roy_specialsend,
        roy_specialsend_fx,
        roy_specialairs1,
        roy_specialairs1_fx,
        roy_specialairs4s,
        roy_specialairs4s_fx,
        roy_final,
        roy_final_fx,
        roy_finalstart_fx,
        roy_finalair,
        roy_finalair_fx,
        roy_finalairstart_fx,
        roy_specialn,
        roy_specialnloop,
        roy_specialnend,
        roy_specialn_fx,
        roy_specialairn,
        roy_specialairn_fx,
        roy_specialairnloop,
        roy_specialairnend
    );
}
