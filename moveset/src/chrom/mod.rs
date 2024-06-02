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

pub const FIGHTER_SAMUS_GENERATE_ARTICLE_LIGHTBEAM :i32 = 0x9;

unsafe extern "C" fn samus_attack11(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_SAMUS_GENERATE_ARTICLE_LIGHTBEAM, false, -1);
    }
}

unsafe extern "C" fn samus_attackairn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_SAMUS_GENERATE_ARTICLE_LIGHTBEAM, false, -1);
    }
}



unsafe extern "C" fn chrom_lightbeam_game_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 20, 0, 35, 2.4, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 15, 0, 28, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 10, 0, 22, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn chrom_lightbeam_effect_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), 0, 0, 0, -90, 0, 0, 1, true);
    }
}


unsafe extern "C" fn effect_appear(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn game_fall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 48, 90, 0, 30, 6.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}


unsafe extern "C" fn samus_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.agent.clear_lua_stack();
    (&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_MOTION as u64));
    fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.66));
    sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    fighter.sub_shift_status_main(L2CValue::Ptr(samus_specialhi_main_loop as *const () as _))
    
}

unsafe extern "C" fn samus_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_frame = fighter.global_table[0xe].get_f32();
    if status_frame >54.0{
        fighter.super_jump_punch_main();
    }
    return 0.into();
}

pub fn install() {
    Agent::new("samus")
        .game_acmd("game_attack11", samus_attack11, Priority::Default)
        .game_acmd("game_attackairn", samus_attackairn, Priority::Default)
        .status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, samus_specialhi_main)
        .install();
    Agent::new("samus_lightbeam")
        .game_acmd("game_fall", game_fall, Priority::Default)
        .effect_acmd("effect_appear", effect_appear, Priority::Default)
        .install();
    /*
    Agent::new("samus_lightbeam")
        .game_acmd("game_break", game_break, Priority::Default)
        .effect_acmd("effect_regular", effect_break, Priority::Default)
        .game_acmd("game_shake", game_shake, Priority::Default)
        .effect_acmd("effect_shake", effect_shake, Priority::Default)
        .install();
*/
}
