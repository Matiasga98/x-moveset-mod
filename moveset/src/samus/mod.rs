use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
		hash40
    },
    smash_script::*,
	smashline::*
};

pub const FIGHTER_SAMUS_GENERATE_ARTICLE_WEB :i32 = 0x9;

unsafe extern "C" fn samus_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    for _ in 0..5 {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 10, 10, 0, 55, 4.0, 0.0, 13.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 367, 10, 0, 70, 4.0, 0.0, 13.0, -6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 40, 10, 0, 50, 4.0, 0.0, 4.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 367, 10, 0, 70, 4.0, 0.0, 4.0, -6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
}
if macros::is_excute(agent) {
    macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 47, 105, 0, 80, 11.5, 0.0, 8.8, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
}
wait(agent.lua_state_agent, 2.0);
if macros::is_excute(agent) {
    AttackModule::clear_all(agent.module_accessor);
}
}



unsafe extern "C" fn samus_attackdash_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_machstamp"), Hash40::new("top"), 0, 7, 5, -90, 0, 160, 1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,0.1,0);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR(agent, 2, 0.059, 0.008, 0);
        macros::BURN_COLOR_FRAME(agent, 4, 2, 0.059, 0.008, 0.9);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR(agent, 2, 0.059, 0.008, 0.9);
        macros::BURN_COLOR_FRAME(agent, 12, 2, 0.059, 0.008, 0);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_machstamp"), false, true);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}


unsafe extern "C" fn samus_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_SAMUS_GENERATE_ARTICLE_WEB, false, -1);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 3.0, 84, 100, 90, 0, 9.5, -1.0, 1.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 3.0, 367, 100, 25, 0, 8.5, 1.0, 0.0, 0.0, Some(1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, -10.0, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 1.3, 84, 100, 90, 0, 9.5, -1.0, 1.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 1.3, 367, 100, 25, 0, 8.5, 1.0, 0.0, 0.0, Some(1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, -10.0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 4.0, 70, 160, 0, 40, 9.5, -1.0, 1.0, 2.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 4.0, 70, 160, 0, 40, 8.5, 1.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


unsafe extern "C" fn samus_attackairhi_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_spin"), Hash40::new("top"), -4, -8, 4.2, 180, 0, 0, 1.25, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,1,1);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_spin"), Hash40::new("top"), -4, -8, 7.5, 180, 0, 0, 1.05, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,1,1);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}





unsafe extern "C" fn samus_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 18.0, 361, 76, 0, 62,
         5.3, 5.8, 0.0, 2.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 18.0, 361, 76, 0, 62,
         5.8, 3.0, 0.0, -1.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 16.0, 361, 76, 0, 62, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 4.0, 4.0);
    }
}


unsafe extern "C" fn samus_attacks4_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 16, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("samus_atk_bomb"), Hash40::new("haver"), 0, 0, 0, 0, 0, -90, 1.45, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_firesword1"),
         Hash40::new("tex_firesword1"), 14, Hash40::new("haver"),
          0.0, 0.0, 1.65, Hash40::new("haver"),
           -0.0, -0.0, 12.4, true, Hash40::new("null"),
            Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0,
             0.0, 0.0, 1.0, 0,
              *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("samus_atk_bomb"), false, true);
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}



unsafe extern "C" fn samus_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 88, 84, 0, 53, 6.5, 0.0, 10.0, 20.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 88, 89, 0, 58, 6.0, 0.0, 20.0, 20.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 88, 92, 0, 62, 5.5, 0.0, 40.0, 20.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 88, 84, 0, 53, 6.5, 0.0, 10.0, 20.0, Some(0.0), Some(2.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 88, 89, 0, 58, 6.0, 0.0, 20.0, 20.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 88, 92, 0, 62, 5.5, 0.0, 40.0, 20.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}


unsafe extern "C" fn samus_attackhi4_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("stick"), 0, 8.5, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
    for n in 15..20 {
        let x = (n-15) as f32;
        //println!("{}",n);
        frame(agent.lua_state_agent, n as f32);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, (x*320.0).sqrt(), 20, 245, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,0.4,0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, (x*245.0).sqrt(), 20, 245, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,0.4,0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, (x*180.0).sqrt(), 20, 245, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,0.4,0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, (x*125.0).sqrt(), 20, 245, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,0.4,0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, (x*80.0).sqrt(), 20, 245, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,0.4,0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, (x*45.0).sqrt(), 20, 245, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,0.4,0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, (x*20.0).sqrt(), 20, 245, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,0.4,0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, (x*5.0).sqrt(), 20, 245, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,0.4,0);
        
        //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, 0, 20, 245, 0, 0, 0.7, true);
        //macros::LAST_EFFECT_SET_COLOR(agent, 1,0.5,0);

        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_ice"), Hash40::new("sys_ice"), Hash40::new("top"), 0, 5, 15, 65, 0, 0, 0.5, true, *EF_FLIP_YZ);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, 5, 10, 245, 180, 0, 0.5, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,0.4,0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, 5, 30, 245, 0, 180, 0.5, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,0.4,0);
        
    }    

    
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_ice"), false, false);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 20, 20, 245, 0, 0, 2.0, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,0.4,0);
    }

}



unsafe extern "C" fn samus_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 8.0);
    for _ in 0..5 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 178, 30, 0, 65, 6.0, 0.0, 5.0, -8.0,  Some(0.0), Some(20.0), Some(-8.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 178, 30, 0, 65, 6.0, 0.0, 5.0, 8.0,  Some(0.0), Some(20.0), Some(8.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 90, 30, 0, 30, 4.8, 0.0, 4.0, 0.0,  Some(0.0), Some(20.0), Some(0.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 37, 192, 0, 51, 10.0, 0.0, 6.4, 0.0,  Some(0.0), Some(20.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let motion_frame = MotionModule::frame(agent.module_accessor);
        println!("entre aca{}",motion_frame);
        AttackModule::clear_all(agent.module_accessor);
    }
}


unsafe extern "C" fn samus_attacklw4_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_spin"), Hash40::new("top"), 0, -4, 0,
        180, 0, 0, 1.3, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_spin"), Hash40::new("top"), 0, -4, 0,
        180, 0, 0, 1.3, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.4,0.4,1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_gbeam_lightning"), Hash40::new("top")   , 0, 0, 0, 0, 0, 0, 2.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_gbeam_lightning"), Hash40::new("arml")  , 0, 0, 0, 0, 0, 0, 2.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_gbeam_lightning"), Hash40::new("armr")  , 0, 0, 0, 0, 0, 0, 2.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_gbeam_lightning"), Hash40::new("bust")  , 0, 0, 0, 0, 0, 0, 2.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_gbeam_lightning"), Hash40::new("kneel") , 0, 0, 0, 0, 0, 0, 2.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_gbeam_lightning"), Hash40::new("kneer") , 0, 0, 0, 0, 0, 0, 2.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_gbeam_lightning"), Hash40::new("havel") , 0, 0, 0, 0, 0, 0, 2.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_gbeam_lightning"), Hash40::new("haver") , 0, 0, 0, 0, 0, 0, 2.0, true);
    }
    frame(agent.lua_state_agent, 23.0);
    macros::EFFECT_OFF_KIND(agent, Hash40::new("samus_gbeam_lightning"), false, false);

}




unsafe extern "C" fn samus_attackhi3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 12.0, 90, 70, 0, 70, 3.0, 1.7, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 12.0, 90, 70, 0, 70, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 12.0, 90, 70, 0, 70, 5.0, 2.3, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 8.0, 90, 70, 0, 70, 2.5, 0.0, 9.0, 12.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        //ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, -1);
        //ArticleModule::shoot(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, false, -1);
        AttackModule::clear(agent.module_accessor, 3, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
}



unsafe extern "C" fn samus_attackhi3_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("ryu_attack_arc"), Hash40::new("ryu_attack_arc"), Hash40::new("top"), 2, 14, 3, -9, -20, 111, 0.65, true, *EF_FLIP_YZ, 0.35);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 7, 19, 2, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, false, 0.4);
    }
}



unsafe extern "C" fn samus_throwf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 42, 55, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(agent, 16, 16);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 5.0, y: 5.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}


unsafe extern "C" fn samus_throwf_fx(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, 20, 
        0, 245, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), -3, 20, 
        0, 290, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 3, 20, 
        0, 200, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, 5, 
        0, 65, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), -3, 5, 
        0, 20, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 3, 5, 
        0, 110, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), -3, 12, 
        0, 335, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 3, 12, 
        0, 155, 0, 0, 0.7, true);
        
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_ice"), false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 12, 0, 245, 0, 0, 2.0, true);
    }
}


unsafe extern "C" fn samus_throwf_ex(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
}




unsafe extern "C" fn samus_specialhi_fx(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samus_screwattack"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1,0.94,0);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("samus_screwattacks"), false, true);
    }
}



unsafe extern "C" fn samus_specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        GROUND_CORRECT_KIND_KEEP.into(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC,
        false, 
        false, 
        false, 
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        FIGHTER_STATUS_ATTR_START_TURN.into(),
        FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI.into(),
        0
    );
    return 0.into();
}


unsafe extern "C" fn samus_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	//ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, false, -1);
    //MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
    /*/
    WorkModule::set_int64(fighter.module_accessor, 0xa28f17495, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, 0xed8a31e01, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND_AIR);
    WorkModule::set_float(fighter.module_accessor, 0.25, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, 0.625, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, 18.0, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, 0.5, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
    WorkModule::set_float(fighter.module_accessor, 0.66, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
    WorkModule::set_float(fighter.module_accessor, 0.95, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, 0.6, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
    WorkModule::set_int(fighter.module_accessor, 30, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_CONST_LANDING_FRAME);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    fighter.super_jump_punch(L2CValue::Void());
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    */
    /* 
    fighter.agent.clear_lua_stack();
    (&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_MOTION as u64));
    fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.66));
    sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    */

    fighter.sub_shift_status_main(L2CValue::Ptr(samus_specialhi_main_loop as *const () as _))
}

unsafe extern "C" fn samus_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_frame = fighter.global_table[0xe].get_f32();
    if (status_frame >55.0){
        //return call_original!()(fighter);
    }
    return 0.into();
}


unsafe extern "C" fn samus_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.super_jump_punch_end(L2CValue::Ptr(L2CFighterCommon_super_jump_punch_reset_common_condition as *const () as _));
    return 0.into();
}



unsafe extern "C" fn samus_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        frame(agent.lua_state_agent, 1.0);
        StatusModule::change_status_request_from_script(agent.module_accessor, FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
    }
}

unsafe extern "C" fn samus_specialairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        frame(agent.lua_state_agent, 1.0);
        StatusModule::change_status_request_from_script(agent.module_accessor, FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
    }
}





unsafe extern "C" fn samus_bomb_game_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        frame(agent.lua_state_agent, 5.0);
        //macros::ATTACK(agent, 0, 0, Hash40::new("top"),  6.0, 80, 70, 0, 70, 5.0, 2.3, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 45, 0, 22, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

unsafe extern "C" fn samus_bomb_game_burst(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    let facing = PostureModule::lr(agent.module_accessor);
       

    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_status = StatusModule::status_kind(owner_boma);
    if owner_status == FIGHTER_STATUS_KIND_ATTACK_HI3 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"),  6.0, 80, 70, 0, 70, 10.0, 2.3, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 8.0);
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
    else{
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"),  6.0, 80, 70, 0, 70, 10.0, 2.3, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 30.0);
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
    
}

unsafe extern "C" fn samus_bomb_game_burst_fx(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, 0, 0,
        // 0, 0, 0, 1.0, true);
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}


unsafe extern "C" fn samus_bomb_start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
	StatusModule::init_settings(
		weapon.module_accessor, 
		smash::app::SituationKind(*SITUATION_KIND_AIR), 
		*WEAPON_KINETIC_TYPE_NORMAL, 
		GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK.into(), 
		smash::app::GroundCliffCheckKind(0), 
		false, 
		0, 
		0, 
		0, 
		0
	);
	return 0.into();
}


unsafe extern "C" fn samus_bomb_start_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
	MotionModule::change_motion(weapon.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
	weapon.fastshift(L2CValue::Ptr(samus_bomb_start_main_loop as *const () as _))
}


unsafe extern "C" fn samus_bomb_start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {

	let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_status = StatusModule::status_kind(owner_boma);
    let facing = PostureModule::lr(weapon.module_accessor);
    if owner_status == FIGHTER_STATUS_KIND_ATTACK_HI3 {
	    let mut speed_x: f32 = 0.0;//lua_bind::KineticEnergy::get_speed_x(energy_type);
        let mut speed_y: f32 = 2.0;//lua_bind::KineticEnergy::get_speed_y(energy_type);
        let status_frame = weapon.global_table[0xe].get_f32();
        ModelModule::set_mesh_visibility(
            weapon.module_accessor,
            Hash40::new("curve"),
            false,
        );
        //VisibilityModule::set_model_visible(weapon.module_accessor, false);
        macros::EFFECT_FOLLOW_NO_STOP(weapon, Hash40::new("sys_machstamp"), Hash40::new("top"),
         0, 0, 0, 180, 0, 0, 0.5, true);
        macros::LAST_EFFECT_SET_COLOR(weapon, 1,0.1,0);
        if status_frame == 1.0 {
            speed_x = if facing == 1.0 { 8.0 } else { -8.00 };
            speed_y = 0.0;
        }
        if status_frame >= 30.0 {
            macros::EFFECT_OFF_KIND(weapon, Hash40::new("sys_machstamp"), false, true);
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            //StatusModule::change_status_request_from_script(weapon.module_accessor, WEAPON_SAMUS_BOMB_STATUS_KIND_BURST_ATTACK.into(), false.into());
        }
        
        weapon.agent.clear_lua_stack();
        weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
        weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_x));
        weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_y));
        sv_kinetic_energy::set_speed(weapon.lua_state_agent);
    }
    else  {
        let mut speed_x: f32 = 0.0;
        let mut speed_y: f32 = 0.0;
        let status_frame = weapon.global_table[0xe].get_f32();
        
        if status_frame == 1.0 {
            //MotionModule::set_rate(owner_boma, 1.5);
            //speed_x = if facing == 1.0 { 8.0 } else { -8.00 };
            speed_y = -20.0;
            VisibilityModule::set_whole(weapon.module_accessor, false);
        }
        if status_frame == 7.0 {
            ModelModule::set_mesh_visibility(
                weapon.module_accessor,
                Hash40::new("curve"),
                true,
            );
            VisibilityModule::set_whole(weapon.module_accessor, true);
        }
        if status_frame >= 15.0 {
            StatusModule::change_status_request_from_script(weapon.module_accessor, WEAPON_SAMUS_BOMB_STATUS_KIND_BURST_ATTACK.into(), false.into());
        }

        
        weapon.agent.clear_lua_stack();
        weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
        weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_x));
        weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_y));
        sv_kinetic_energy::set_speed(weapon.lua_state_agent);
    }
	
    return 0.into();
}


unsafe extern "C" fn samus_bomb_fall_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}



unsafe extern "C" fn samus_bomb_hburst_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor,1, *WEAPON_SAMUS_BOMB_INSTANCE_WORK_ID_INT_BOMBJUMP);
    return 0.into();
}






 
 unsafe extern "C" fn samus_specialairs(agent: &mut L2CAgentBase) {
    println!("entre aca");
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(agent, 0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("mouth2"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_XLU);
    }
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(agent, 0, -4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 275, 100, 0, 30, 9.0, 0.0, -3.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 56, 100, 0, 30, 9.0, 0.0, -3.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
 }
 
 


unsafe extern "C" fn samus_supermissile_start_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
	MotionModule::change_motion(weapon.module_accessor, Hash40::new("ready"), 0.0, 1.0, false, 0.0, false, false);
	//IS_CONTROLLING_MISSILE = true;
    weapon.fastshift(L2CValue::Ptr(samus_supermissile_start_main_loop as *const () as _))
}



unsafe extern "C" fn samus_supermissile_start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
	// Declare owner boma
	let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	// Declare facing
    if StatusModule::status_kind(owner_boma)==FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    else {       
        let facing = PostureModule::lr(weapon.module_accessor);
        // Declare x and y speeds
        let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
        let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);
        let mut speed_y: f32 = lua_bind::KineticEnergy::get_speed_y(energy_type);
        let mut rot = lua_bind::KineticEnergy::get_rotation(energy_type);

        let status_frame = weapon.global_table[0xe].get_f32();
        // Get control stick y pos
        let stick_y = ControlModule::get_stick_y(owner_boma);
        let stick_x = ControlModule::get_stick_x(owner_boma);
        let mut new_speed : f32 = 2.0;
        //println!("init speed_x : {}",speed_x);
        //println!("init speed_y : {}",speed_y);
        let owner_status = StatusModule::status_kind(owner_boma);
        if stick_y == 0.0 && stick_x == 0.0 {

        }
        else if owner_status == FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G{
            
            
            if stick_y.abs() > stick_x.abs() && speed_y == 0.0{
                speed_x = 0.0;
                speed_y = if stick_y > 0.0 {new_speed}else {new_speed*-1.0};
            }
            else if stick_y.abs() < stick_x.abs() && speed_x == 0.0 {
                speed_x = if stick_x > 0.0 {new_speed}else {new_speed*-1.0};
                speed_y = 0.0;
            }
        }
        if status_frame <= 15.0 {
            speed_x = if facing == 1.0 { 2.0 } else { -2.00 };
            speed_y = 0.0;
            //WAS_SLOW_BEFORE_MISSILE = SlowModule::is_slow(owner_boma);
            //MotionModule::set_rate(owner_boma, 0.0);
            //SlowModule::set(owner_boma, 0, 60, 180, true, 0);
            //StatusModule::change_status_request_from_script(owner_boma,FIGHTER_STATUS_KIND_DEMO.into(),false.into());
        }
        if status_frame == 160.0 || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32){
            StatusModule::change_status_request_from_script(weapon.module_accessor, WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_S_BURST.into(), false.into());
        }
        // Set speed
        weapon.agent.clear_lua_stack();
        weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
        weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_x));
        weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_y));
        sv_kinetic_energy::set_speed(weapon.lua_state_agent);
        ModelModule::set_joint_rotate(weapon.module_accessor,Hash40::new("rot"), 
        &Vector3f {
            x: speed_y*-45.0,
            y:  if speed_x*facing>0.0 {0.0} else {180.0},
            z: 0.0,
        }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }    
	return 0.into();
}


static mut supermissile_frames: i32 = 0;
static mut supermissile_alive: bool = false;
unsafe extern "C" fn supermissile_frame(weapon: &mut L2CFighterBase)  {
    unsafe {
        let supermissile_status = StatusModule::status_kind(weapon.module_accessor);
        if supermissile_status == WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_READY {
            supermissile_alive = true;
        }
        else{
            supermissile_alive = false;
        }
    }
}



unsafe extern "C" fn samus_missile_game_homing(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 70, 40, 0, 80,
         7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    
}


unsafe extern "C" fn samus_missile_game_hburst(agent: &mut L2CAgentBase) {
    
}


unsafe extern "C" fn samus_missile_start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //println!("pre homing");
	StatusModule::init_settings(
		weapon.module_accessor, 
		smash::app::SituationKind(*SITUATION_KIND_AIR), 
		*WEAPON_KINETIC_TYPE_NORMAL, 
		GROUND_CORRECT_KIND_AIR.into(), 
		smash::app::GroundCliffCheckKind(0), 
		false, 
		0, 
		0, 
		0, 
		0
	);
    //StatusModule::change_status_request_from_script(weapon.module_accessor, WEAPON_SAMUS_MISSILE_STATUS_KIND_H_BURST.into(), false.into());
	return 0.into();
}


unsafe extern "C" fn samus_missile_start_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //MotionModule::change_motion(weapon.module_accessor, Hash40::new("homing"), 0.0, 1.0, false, 0.0, false, false);
	//println!("main homing");
    weapon.fastshift(L2CValue::Ptr(samus_missile_start_main_loop as *const () as _))
}
unsafe extern "C" fn samus_missile_start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //println!("main loop homing");
    return 0.into();
}


unsafe extern "C" fn samus_missile_start_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //println!("exec homing");
	MotionModule::change_motion(weapon.module_accessor, Hash40::new("homing"), 0.0, 1.0, false, 0.0, false, false);
	//StatusModule::change_status_request_from_script(weapon.module_accessor, WEAPON_SAMUS_MISSILE_STATUS_KIND_H_BURST.into(), false.into());
    missile_alive = true;
    weapon.fastshift(L2CValue::Ptr(samus_missile_start_exec_loop as *const () as _))
}


static mut missile_x: f32 = 0.0;
static mut missile_y: f32 = 0.0;
static mut missile_alive: bool = false;
static mut missile_frames: i32 = 0;
unsafe extern "C" fn samus_missile_start_exec_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //println!("exec loop homing");
    missile_x = PostureModule::pos_x(weapon.module_accessor);
    missile_y = PostureModule::pos_y(weapon.module_accessor);
    let facing: f32 = PostureModule::lr(weapon.module_accessor);
    let status_frame: f32 = weapon.global_table[0xe].get_f32();
    let mut speed_x: f32 = if facing == 1.0 { 2.0 } else { -2.0 };
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32){
        PostureModule::reverse_lr(weapon.module_accessor);
        speed_x = -speed_x;
    }
    if status_frame == 180.0 {
		StatusModule::change_status_request_from_script(weapon.module_accessor, WEAPON_SAMUS_MISSILE_STATUS_KIND_H_BURST.into(), false.into());
	}

    missile_rotation += 10.0;
    ModelModule::set_joint_rotate(weapon.module_accessor,Hash40::new("rot"), 
    &Vector3f {
        x: missile_rotation,
        y: 0.0,
        z: 0.0,
    }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    
    //StatusModule::change_status_request_from_script(weapon.module_accessor, WEAPON_SAMUS_MISSILE_STATUS_KIND_H_BURST.into(), false.into());
    weapon.agent.clear_lua_stack();
	weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
	weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_x));
	weapon.agent.push_lua_stack(&mut L2CValue::new_num(-1.0));
	sv_kinetic_energy::set_speed(weapon.lua_state_agent);
    return 0.into();
}

//unsafe extern "C" fn wolf_special_s_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // If flag is on, activate damage multiplier and turn flag off
    /*if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SPECIAL_S_COMMAND) {
        AttackModule::set_power_up(fighter.module_accessor, 1.5);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SPECIAL_S_COMMAND)
    }*/
    // Run original code of special_s status
    //original!(fighter)
//}

unsafe extern "C" fn samus_missile_hburst_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //println!("pre hburst");
	StatusModule::init_settings(
		weapon.module_accessor, 
		smash::app::SituationKind(*SITUATION_KIND_AIR), 
		*WEAPON_KINETIC_TYPE_NORMAL, 
		GROUND_CORRECT_KIND_AIR.into(), 
		smash::app::GroundCliffCheckKind(0), 
		false, 
		*WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG, 
		*WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT, 
		*WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 
		0
	);
	return 0.into();
}



unsafe extern "C" fn samus_missile_hburst_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
	//MotionModule::change_motion(weapon.module_accessor, Hash40::new("h_burst"), 0.0, 1.0, false, 0.0, false, false);
	MotionModule::change_motion(weapon.module_accessor, Hash40::new("homing"), 0.0, 1.0, false, 0.0, false, false);
    //println!("main hburst");
    missile_alive = false;
    weapon.fastshift(L2CValue::Ptr(samus_missile_hburst_main_loop as *const () as _))
}

static mut missile_rotation: f32 = 0.0;
unsafe extern "C" fn samus_missile_hburst_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
	let facing = PostureModule::lr(weapon.module_accessor);
	let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
	let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);
	let mut speed_y: f32 = lua_bind::KineticEnergy::get_speed_y(energy_type);
	let status_frame = weapon.global_table[0xe].get_f32();
    //println!("frame {}",status_frame);
    if status_frame == 1.0 {
        speed_x = if facing == 1.0 { 0.0 } else { 0.0 };
        speed_y = 1.0;
        println!("========================================== ")
    }
    else{
        let mut accel_x : f32 = 0.0;
        let mut accel_y : f32 = 0.0;
        let variable = ((status_frame-2.0) / 20.0).rem_euclid(4.0).trunc();
        //println!("frame : {}",status_frame);
        //println!("variable : {}",variable);
        if status_frame >= 62.0 {
            let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
            let projectile_x =  PostureModule::pos_x(weapon.module_accessor);
            let projectile_y =  PostureModule::pos_y(weapon.module_accessor);
            let owner_x =  PostureModule::pos_x(owner_boma);
            let owner_y =  PostureModule::pos_y(owner_boma)+8.0; //pos_y is feet, 2.0 to go to middle

            let dist_x = owner_x - projectile_x;
            let dist_y = owner_y - projectile_y;
            let dist = (dist_x*dist_x + dist_y*dist_y).sqrt();
            
            if dist < 4.0 
            {
                //this removes a projectile
                notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            }

            let new_speed_x = 1.5*(dist_x/dist);
            let new_speed_y = 1.5*(dist_y/dist);

            if speed_x.abs() - new_speed_x.abs() > 0.5 { speed_x = speed_x + 0.5*new_speed_x}
            else {speed_x = speed_x + new_speed_x}
            if speed_y.abs() - new_speed_y.abs() > 0.5 { speed_y = speed_y + 0.5*new_speed_y}
            else {speed_y = speed_y + new_speed_y}
            
            if speed_x > 2.0 {speed_x = 2.0}
            if speed_x < -2.0 {speed_x = -2.0}
            
            if speed_y > 2.0 {speed_y = 2.0}
            if speed_y < -2.0 {speed_y = -2.0}
            
        }
        else {
            match variable{ 
                0.0=> accel_y = -0.05,
                1.0=> accel_y = -0.05,
                2.0=> accel_y = 0.05,
                3.0=> accel_y = 0.05,
                _  => println!("Something went wrong"),
            }
            match variable{ 
                0.0=> accel_x = 0.2,
                1.0=> accel_x = -0.2,
                2.0=> accel_x = -0.2,
                3.0=> accel_x = 0.2,
                _  => println!("Something went wrong"),
            }
            speed_x = speed_x + accel_x;
            speed_y = speed_y + accel_y;
        }
        
        //println!("speed_x : {}",speed_x);
        //println!("speed_y : {}",speed_y);
    }
	if status_frame == 566.0 {
		StatusModule::change_status_request_from_script(weapon.module_accessor, WEAPON_SAMUS_MISSILE_STATUS_KIND_H_BURST.into(), false.into());
	}
    
    // Spinny boi
    missile_rotation += 10.0;
    ModelModule::set_joint_rotate(weapon.module_accessor,Hash40::new("rot"), 
    &Vector3f {
        x: missile_rotation,
        y: 0.0,
        z: 0.0,
    }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    
	// Set speed
    //println!("speed_x : {}",speed_x);
    let value = WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_INT_LIFE);
    //    println!("value : {}",value);
	weapon.agent.clear_lua_stack();
	weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
	weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_x));
	weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_y));
	sv_kinetic_energy::set_speed(weapon.lua_state_agent);
	return 0.into();
}



static mut previ_status: i32 = 50;
static mut prev_motion: u64 = 50;

unsafe extern "C" fn missile_frame(weapon: &mut L2CFighterBase)  {
    unsafe {
        let missile_status = StatusModule::status_kind(weapon.module_accessor);
        //println!("status : {}",missile_status);
        if missile_status == WEAPON_SAMUS_MISSILE_STATUS_KIND_H_BURST {
            //println!("IS IN HBURST");
        }
        if missile_status != previ_status{
            //println!("status : {}",missile_status);
        }
        let missile_kind = MotionModule::motion_kind(weapon.module_accessor);
        if missile_kind != prev_motion {
            //println!("motion : {}",missile_kind);
        }
        previ_status = missile_status;
        prev_motion = missile_kind;
        
        //WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_INT_LIFE);
    }
}


static mut cshot_x: f32 = 0.0;
static mut cshot_y: f32 = 0.0;
static mut cshot_shooting: bool = false;
static mut cshot_frames: i32 = 0;
static mut cshot_charge : f32 = 0.0;
static mut cshot_effect : u64  = 0;
static mut cshot_msg : u64 = 0;

unsafe extern "C" fn cshot_frame(weapon: &mut L2CFighterBase)  {
    unsafe {
        let cshot_status = StatusModule::status_kind(weapon.module_accessor);
        println!("status : {}",cshot_status);
        cshot_charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
        println!("charge : {}",cshot_charge);
        cshot_x = PostureModule::pos_x(weapon.module_accessor);
        cshot_y = PostureModule::pos_y(weapon.module_accessor);

        EffectModule::kill_kind(weapon.module_accessor,  Hash40::new("samus_cshot_shot"), true, true);
        ArticleModule::set_visibility_whole(weapon.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        if cshot_status == 1{
            cshot_msg = 1;
            cshot_shooting = true;
            //EffectModule::kill_kind(weapon.module_accessor,  Hash40::new("sys_machstamp"), true, true);
            let size = match cshot_charge{
                c if c < 0.5             => 0.2,
                c if c >= 0.5 && c < 1.0 => 0.5,
                c if c >=1.0             => 1.0,
                _                             => 20.0 
            };
            if EffectModule::is_exist_effect(weapon.module_accessor,cshot_effect as u32){
                EffectModule::set_pos(weapon.module_accessor, cshot_effect as u32, &Vector3f{x: cshot_x, y: cshot_y, z: 0.0});
            }
            else{
                let facing = PostureModule::lr(weapon.module_accessor);
                cshot_effect = EffectModule::req_2d(weapon.module_accessor,  Hash40::new("sys_machstamp"), &Vector3f{x: cshot_x, y: cshot_y, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 1.55*facing}, size, 0);
            }
            match cshot_charge{ 
                c if c < 0.5            => EffectModule::set_rgb(weapon.module_accessor,cshot_effect as u32, 1.0,1.0,0.0),
                c if c >= 0.5 && c < 1.0 => EffectModule::set_rgb(weapon.module_accessor,cshot_effect as u32, 0.3,1.0,0.3),
                c if c >=1.0            => EffectModule::set_rgb(weapon.module_accessor,cshot_effect as u32, 0.5,0.5,1.0),
                _                            => println!("Something went wrong"),
            }; 
        }
        else{
            EffectModule::kill_kind(weapon.module_accessor,  Hash40::new("sys_machstamp"), true, true);
            cshot_shooting = false;
        }
        println!("is shooting : {}",cshot_shooting);
        
    }
}



unsafe extern "C" fn samus_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        //if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI){
        //    StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP.into(), false.into());
        //}
        let x_status = StatusModule::status_kind(fighter.module_accessor);
        let prev_status = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        let prev_prev_status = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        let prev3_status = StatusModule::prev_status_kind(fighter.module_accessor, 2);
        let prev4_status = StatusModule::prev_status_kind(fighter.module_accessor, 3);
        let situation = StatusModule::situation_kind(fighter.module_accessor);
        let motion = MotionModule::motion_kind(fighter.module_accessor);
        let motion_frame = MotionModule::frame(fighter.module_accessor);

        
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        if cshot_shooting{
            if cshot_msg == 1 {
                cshot_msg = 0;
            }
            else {
                cshot_shooting = false;
                cshot_msg = 0;
                EffectModule::kill_kind(fighter.module_accessor,  Hash40::new("sys_machstamp"), true, true);
                EffectModule::kill(fighter.module_accessor, cshot_effect as u32, true, true);
            }
        }

        if motion == smash::hash40("special_air_s") && motion_frame <= 70.0{
            show_mesh("anvil1",fighter);
            show_mesh("anvil2",fighter);
            show_mesh("handmesh_r_c",fighter);
            show_mesh("handmesh_r_m",fighter);
            hide_mesh("buster_009_u",fighter);
        }
        else {
            hide_mesh("anvil1",fighter);
            hide_mesh("anvil2",fighter);
            hide_mesh("handmesh_r_c",fighter);
            hide_mesh("handmesh_r_m",fighter);
            show_mesh("buster_009_u",fighter);
        }
        if (motion == smash::hash40("attack_s4_s") && motion_frame <= 30.0) || motion == smash::hash40("attack_s4_hold") {
            println!("forward smash");
            show_mesh("sword1",fighter);
            show_mesh("sword2",fighter);
            show_mesh("sword3",fighter);
            show_mesh("handmesh_r_c",fighter);
            show_mesh("handmesh_r_m",fighter);
            hide_mesh("buster_009_u",fighter);
        }
        else{
            hide_mesh("sword1",fighter);
            hide_mesh("sword2",fighter);
            hide_mesh("sword3",fighter);
            hide_mesh("handmesh_r_c",fighter);
            hide_mesh("handmesh_r_m",fighter);
            show_mesh("buster_009_u",fighter);
        }

        //print_statuses(fighter);

        if missile_alive && missile_frames < 180{
            //println!("creating shield");
            let pos_x = PostureModule::pos_x(fighter.module_accessor);
            let pos_y = PostureModule::pos_y(fighter.module_accessor);
            let facing = PostureModule::lr(fighter.module_accessor);
            missile_frames = missile_frames + 1 ;
            shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_SHIELD, 
                0, // ID
                Hash40::new("top"), // Bone
                10.0, // Size
                0.0, missile_y-pos_y, (missile_x-pos_x+facing*20.0)*facing,//missile_x-pos_x, // X1, Y1, Z1
                0.0, missile_y-pos_y, (missile_x-pos_x+facing*20.0)*facing,//missile_x-pos_x, // X2, Y2, Z2
                4.0, // Damage multiplier
                0.1, // Speed multiplier
                200, // Max damage
                false, // Unknown
                20.0, // Life multiplier
                *FIGHTER_SHIELD_GROUP_KIND_GUARD);
            
        }
        else{
            shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
            missile_frames = 0;
        }

        if supermissile_frames >0{
            if supermissile_alive {
                supermissile_frames-=1;
                if supermissile_frames == 140{
                    MotionModule::set_rate(fighter.module_accessor, 0.0);
                }
                else if supermissile_frames == 0 {
                    supermissile_alive = false;
                    MotionModule::set_rate(fighter.module_accessor, 1.0);
                }
            }
            else if MotionModule::rate(fighter.module_accessor)==0.0{
                MotionModule::set_rate(fighter.module_accessor, 1.0);
            }
            
        }
        
        

        if should_go_into_web_jump(x_status,prev_status,prev_prev_status) {
            println!("BOMB JUMP INTO SP HI");
            StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_STATUS_KIND_SPECIAL_HI.into(), false.into());
        }
        else if should_go_into_flash_laser(x_status, prev_status, prev_prev_status) {
            println!("going to flash laser");
            StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G.into(), false.into());
            supermissile_frames = 160;
        }
        else if should_go_into_metal_anchor(x_status, prev_status, prev_prev_status) {
            println!("going to metal anchor");
            StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A.into(), false.into());
        }
        else if should_cancel_metal_anchor(x_status,prev_status,situation) {
            println!("cancelling metal anchor");
            StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR.into(), false.into());
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if should_slow_metal_anchor_landing(x_status, prev_status) {
            println!("SLOWING DOWN METAL ANCHOR");
            MotionModule::set_rate(fighter.module_accessor, 0.5);
        }
        
        else if should_go_into_rolling_shield(x_status, prev_status, prev_prev_status){
            println!("going to rolling shield");
            if situation == SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G.into(), false.into());
            }
            else {
                StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A.into(), false.into());
            }
        }
        
        else if should_go_into_web_shoot(x_status, prev_status, prev_prev_status){
            println!("SP HI INTO SP LW");
            StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
            
            //MotionModule::set_rate(fighter.module_accessor, 1.5);
        }
        
    
        
        //println!("status : {}",x_status);
        //println!("motion : {}",MotionModule::motion_kind(fighter.module_accessor) );
        
    }
}
//status : 208
//FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP
//motion : 26486891585
//status : 444
//FIGHTER_STATUS_KIND_GIMMICK_JUMP_BOARD_JUMP
//motion : 66644525903

unsafe extern "C" fn samus_gbeam_catch(agent: &mut L2CAgentBase) {
    
}



pub fn install() {
    Agent::new("samus")
        .game_acmd("game_attackdash", samus_attackdash, Priority::Low)
        .effect_acmd("effect_attackdash", samus_attackdash_fx, Priority::Low)
        .game_acmd("game_attackairhi", samus_attackairhi, Priority::Low)
        .effect_acmd("effect_attackairhi", samus_attackairhi_fx, Priority::Low)
        .game_acmd("game_attacks4", samus_attacks4, Priority::Low)
        .effect_acmd("effect_attacks4", samus_attacks4_fx, Priority::Low)
        .game_acmd("game_attackhi4", samus_attackhi4, Priority::Low)
        .effect_acmd("effect_attackhi4", samus_attackhi4_fx, Priority::Low)
        .game_acmd("game_attacklw4", samus_attacklw4, Priority::Low)
        .effect_acmd("effect_attacklw4", samus_attacklw4_fx, Priority::Low)
        .game_acmd("game_attackhi3", samus_attackhi3, Priority::Low)
        .effect_acmd("effect_attackhi3", samus_attackhi3_fx, Priority::Low)
        .game_acmd("game_throwf", samus_throwf, Priority::Low)
        .effect_acmd("effect_throwf", samus_throwf_fx, Priority::Low)
        .expression_acmd("expression_throwf", samus_throwf_ex, Priority::Low)
        .effect_acmd("effect_specialairhi", samus_specialhi_fx, Priority::Low)
        .game_acmd("game_specialairs", samus_specialairs, Priority::Low)
        .on_line(smashline::Main, samus_frame)
        .install();
    Agent::new("samus_bomb")
        .game_acmd("game_fall", samus_bomb_game_regular, Priority::Low)
        .game_acmd("game_burstattack", samus_bomb_game_burst, Priority::Low)
        .effect_acmd("effect_burstattack", samus_bomb_game_burst_fx, Priority::Low)
        .status(smashline::Pre, *WEAPON_SAMUS_BOMB_STATUS_KIND_FALL, samus_bomb_start_pre)
        .status(smashline::Main, *WEAPON_SAMUS_BOMB_STATUS_KIND_FALL, samus_bomb_start_main)
        .status(smashline::Init, *WEAPON_SAMUS_BOMB_STATUS_KIND_FALL, samus_bomb_fall_init)
        .status(smashline::Init, *WEAPON_SAMUS_BOMB_STATUS_KIND_BURST_ATTACK, samus_bomb_hburst_init)
        .install();
    Agent::new("samus_supermissile")
        .status(smashline::Main, *WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_READY, samus_supermissile_start_main)
        .on_line(smashline::Main, supermissile_frame)
        .install();
    Agent::new("samus_missile")
        .game_acmd("game_homing", samus_missile_game_homing, Priority::Low)
        .game_acmd("game_hburst", samus_missile_game_hburst, Priority::Low)
        .status(smashline::Pre, *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, samus_missile_start_pre)
        .status(smashline::Main, *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, samus_missile_start_main)
        .status(smashline::Exec, *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, samus_missile_start_exec)
        .status(smashline::Pre, *WEAPON_SAMUS_MISSILE_STATUS_KIND_H_BURST, samus_missile_hburst_pre)
        .status(smashline::Main, *WEAPON_SAMUS_MISSILE_STATUS_KIND_H_BURST, samus_missile_hburst_main)
        .on_line(smashline::Main, missile_frame)
        .install();
    Agent::new("samus_cshot")
        .on_line(smashline::Main, cshot_frame)
        .install();
    Agent::new("samus_gbeam")
        .effect_acmd("effect_catch", samus_gbeam_catch, Priority::Low)
	.install();
}



pub fn show_mesh(mesh_name : &str, fighter : &mut L2CFighterCommon){
    unsafe { 
        ModelModule::set_mesh_visibility(
        fighter.module_accessor,
        Hash40::new(mesh_name),
        true,) 
    };
}
pub fn hide_mesh(mesh_name :  &str, fighter : &mut L2CFighterCommon){
    unsafe { 
        ModelModule::set_mesh_visibility(
        fighter.module_accessor,
        Hash40::new(mesh_name),
        false,) 
    };
}

pub fn print_statuses(fighter : &mut L2CFighterCommon ){
    unsafe {
    let x_status = StatusModule::status_kind(fighter.module_accessor);
    let prev_status = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    let prev_prev_status = StatusModule::prev_status_kind(fighter.module_accessor, 1);
    let prev3_status = StatusModule::prev_status_kind(fighter.module_accessor, 2);
    let prev4_status = StatusModule::prev_status_kind(fighter.module_accessor, 3);
    let situation = StatusModule::situation_kind(fighter.module_accessor);
    println!("================================");
    println!("Status {}",x_status);
    println!("PrevStatus {}",prev_status);
    println!("PrevStatus 2 {}",prev3_status);
    println!("PrevStatus 3 {}",prev4_status);   
    println!("situation {}",situation);  
    println!("================================");
    }
}


pub fn should_go_into_web_jump(status : i32, prev_status : i32, prev_prev_status:i32) -> bool {
    return status == FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A;
}
pub fn should_go_into_web_shoot(status : i32, prev_status : i32, prev_prev_status:i32) -> bool {
    return status == FIGHTER_STATUS_KIND_SPECIAL_HI && prev_status != FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A;
}

//unused for now
pub fn should_go_into_boomerang(status : i32, prev_status : i32, prev_prev_status:i32) -> bool {
    return (status == FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW 
        && prev_prev_status != FIGHTER_STATUS_KIND_SPECIAL_HI 
        && prev_status != FIGHTER_STATUS_KIND_SPECIAL_HI)
        || (status == FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW 
            && prev_status != FIGHTER_STATUS_KIND_SPECIAL_HI);
    
}


pub fn should_go_into_rolling_shield(status : i32, prev_status : i32, prev_prev_status:i32) -> bool {
    return (status == FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A && prev_status != FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW ) ||
     (status == FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G && prev_status != FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW )
       
    
}

pub fn should_go_into_flash_laser(status : i32, prev_status : i32, prev_prev_status:i32) -> bool {
    return  (status == FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW 
            && prev_status != FIGHTER_STATUS_KIND_SPECIAL_HI);
    
}


pub fn should_go_into_metal_anchor(status : i32, prev_status : i32, prev_prev_status:i32) -> bool {
    return  status == FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW 
        && prev_prev_status != FIGHTER_STATUS_KIND_SPECIAL_HI 
        && prev_status != FIGHTER_STATUS_KIND_SPECIAL_HI;
    
}

pub fn should_cancel_metal_anchor(status : i32, prev_status : i32, situation:i32) -> bool {
    return  status == FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A 
        && prev_status == FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW
        && situation == SITUATION_KIND_GROUND;
}

pub fn should_slow_metal_anchor_landing(status : i32, prev_status : i32) -> bool {
    return  status == FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR && prev_status ==FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A;
}
