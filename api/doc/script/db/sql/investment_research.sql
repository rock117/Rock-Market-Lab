--
-- PostgreSQL database dump
--

-- Dumped from database version 17.5 (Debian 17.5-1.pgdg120+1)
-- Dumped by pg_dump version 17.5

-- Started on 2025-07-28 17:16:27

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 217 (class 1259 OID 24653)
-- Name: balancesheet; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.balancesheet (
    ts_code character varying(30) NOT NULL,
    ann_date character varying(30) NOT NULL,
    f_ann_date character varying(30) NOT NULL,
    end_date character varying(30) NOT NULL,
    report_type character varying(30),
    comp_type character varying(30),
    end_type character varying(30),
    total_share numeric,
    cap_rese numeric,
    undistr_porfit numeric,
    surplus_rese numeric,
    special_rese numeric,
    money_cap numeric,
    trad_asset numeric,
    notes_receiv numeric,
    accounts_receiv numeric,
    oth_receiv numeric,
    prepayment numeric,
    div_receiv numeric,
    int_receiv numeric,
    inventories numeric,
    amor_exp numeric,
    sett_rsrv numeric,
    loanto_oth_bank_fi numeric,
    premium_receiv numeric,
    reinsur_receiv numeric,
    reinsur_res_receiv numeric,
    pur_resale_fa numeric,
    oth_cur_assets numeric,
    total_cur_assets numeric,
    fa_avail_for_sale numeric,
    htm_invest numeric,
    lt_eqt_invest numeric,
    invest_real_estate numeric,
    time_deposits numeric,
    oth_assets numeric,
    lt_rec numeric,
    fix_assets numeric,
    cip numeric,
    const_materials numeric,
    fixed_assets_disp numeric,
    produc_bio_assets numeric,
    oil_and_gas_assets numeric,
    intan_assets numeric,
    r_and_d numeric,
    goodwill numeric,
    lt_amor_exp numeric,
    defer_tax_assets numeric,
    decr_in_disbur numeric,
    oth_nca numeric,
    total_nca numeric,
    cash_reser_cb numeric,
    depos_in_oth_bfi numeric,
    prec_metals numeric,
    deriv_assets numeric,
    rr_reins_une_prem numeric,
    rr_reins_outstd_cla numeric,
    rr_reins_lins_liab numeric,
    rr_reins_lthins_liab numeric,
    refund_depos numeric,
    ph_pledge_loans numeric,
    refund_cap_depos numeric,
    indep_acct_assets numeric,
    client_depos numeric,
    client_prov numeric,
    transac_seat_fee numeric,
    invest_as_receiv numeric,
    total_assets numeric,
    lt_borr numeric,
    st_borr numeric,
    cb_borr numeric,
    depos_ib_deposits numeric,
    loan_oth_bank numeric,
    trading_fl numeric,
    notes_payable numeric,
    acct_payable numeric,
    adv_receipts numeric,
    sold_for_repur_fa numeric,
    comm_payable numeric,
    payroll_payable numeric,
    taxes_payable numeric,
    int_payable numeric,
    div_payable numeric,
    oth_payable numeric,
    acc_exp numeric,
    deferred_inc numeric,
    st_bonds_payable numeric,
    payable_to_reinsurer numeric,
    rsrv_insur_cont numeric,
    acting_trading_sec numeric,
    acting_uw_sec numeric,
    oth_cur_liab numeric,
    total_cur_liab numeric,
    bond_payable numeric,
    lt_payable numeric,
    specific_payables numeric,
    estimated_liab numeric,
    defer_tax_liab numeric,
    defer_inc_non_cur_liab numeric,
    oth_ncl numeric,
    total_ncl numeric,
    depos_oth_bfi numeric,
    deriv_liab numeric,
    depos numeric,
    agency_bus_liab numeric,
    oth_liab numeric,
    prem_receiv_adva numeric,
    depos_received numeric,
    ph_invest numeric,
    reser_une_prem numeric,
    reser_outstd_claims numeric,
    reser_lins_liab numeric,
    reser_lthins_liab numeric,
    indept_acc_liab numeric,
    pledge_borr numeric,
    indem_payable numeric,
    policy_div_payable numeric,
    total_liab numeric,
    treasury_share numeric,
    ordin_risk_reser numeric,
    forex_differ numeric,
    invest_loss_unconf numeric,
    minority_int numeric,
    total_hldr_eqy_exc_min_int numeric,
    total_hldr_eqy_inc_min_int numeric,
    total_liab_hldr_eqy numeric,
    lt_payroll_payable numeric,
    oth_comp_income numeric,
    oth_eqt_tools numeric,
    oth_eqt_tools_p_shr numeric,
    lending_funds numeric,
    acc_receivable numeric,
    st_fin_payable numeric,
    payables numeric,
    hfs_assets numeric,
    hfs_sales numeric,
    cost_fin_assets numeric,
    fair_value_fin_assets numeric,
    cip_total numeric,
    oth_pay_total numeric,
    long_pay_total numeric,
    debt_invest numeric,
    oth_debt_invest numeric,
    oth_eq_invest numeric,
    oth_illiq_fin_assets numeric,
    oth_eq_ppbond numeric,
    receiv_financing numeric,
    use_right_assets numeric,
    lease_liab numeric,
    contract_assets numeric,
    contract_liab numeric,
    accounts_receiv_bill numeric,
    accounts_pay numeric,
    oth_rcv_total numeric,
    fix_assets_total numeric,
    update_flag character varying(30)
);


--
-- TOC entry 218 (class 1259 OID 24658)
-- Name: dc_index; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.dc_index (
    ts_code character varying(20) NOT NULL,
    trade_date character varying(20) NOT NULL,
    name character varying(100),
    "leading" character varying(100),
    leading_code character varying(20),
    pct_change numeric,
    leading_pct numeric,
    total_mv numeric,
    turnover_rate numeric,
    up_num integer,
    down_num integer
);


--
-- TOC entry 219 (class 1259 OID 24663)
-- Name: margin; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.margin (
    trade_date character varying(20) NOT NULL,
    exchange_id character varying(10) NOT NULL,
    rzye numeric,
    rzmre numeric,
    rzche numeric,
    rqye numeric,
    rqmcl numeric,
    rzrqye numeric,
    rqyl numeric
);


--
-- TOC entry 3421 (class 0 OID 0)
-- Dependencies: 219
-- Name: TABLE margin; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.margin IS '融资融券交易汇总';


--
-- TOC entry 220 (class 1259 OID 24668)
-- Name: stk_holdertrade; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.stk_holdertrade (
    ts_code character varying(25) NOT NULL,
    ann_date character varying(25) NOT NULL,
    holder_name character varying(125) NOT NULL,
    holder_type character varying(25),
    in_de character varying(25) NOT NULL,
    change_vol numeric,
    change_ratio numeric,
    after_share numeric,
    after_ratio numeric,
    avg_price numeric,
    total_share numeric,
    begin_date character varying(25),
    close_date character varying(25)
);


--
-- TOC entry 3422 (class 0 OID 0)
-- Dependencies: 220
-- Name: TABLE stk_holdertrade; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.stk_holdertrade IS '股东增减持';


--
-- TOC entry 221 (class 1259 OID 24673)
-- Name: stock; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.stock (
    ts_code character varying(200) NOT NULL,
    symbol character varying(200) NOT NULL,
    name character varying(200),
    area character varying(200),
    industry character varying(200),
    fullname character varying(200),
    enname character varying(200),
    cnspell character varying(200),
    market character varying(200),
    exchange character varying(200),
    curr_type character varying(200),
    list_status character varying(200),
    list_date character varying(200),
    delist_date character varying(200),
    is_hs character varying(200),
    act_name character varying(200),
    act_ent_type character varying(200),
    name_py character varying(200)
);


--
-- TOC entry 222 (class 1259 OID 24678)
-- Name: stock_daily; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.stock_daily (
    ts_code character varying(45) NOT NULL,
    trade_date character varying(45) NOT NULL,
    open numeric NOT NULL,
    high numeric NOT NULL,
    low numeric NOT NULL,
    close numeric NOT NULL,
    pre_close numeric,
    change numeric,
    pct_chg numeric,
    vol numeric NOT NULL,
    amount numeric NOT NULL
);


--
-- TOC entry 223 (class 1259 OID 24683)
-- Name: stock_daily_basic; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.stock_daily_basic (
    ts_code character varying(200) NOT NULL,
    trade_date character varying(200) NOT NULL,
    close numeric,
    turnover_rate numeric,
    turnover_rate_f numeric,
    volume_ratio character varying(200),
    pe character varying(200),
    pe_ttm character varying(200),
    pb character varying(200),
    ps character varying(200),
    ps_ttm character varying(200),
    dv_ratio character varying(200),
    dv_ttm character varying(200),
    total_share character varying(200),
    float_share character varying(200),
    free_share character varying(200),
    total_mv numeric,
    circ_mv numeric
);


--
-- TOC entry 224 (class 1259 OID 24688)
-- Name: ths_daily; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.ths_daily (
    ts_code character varying(20) NOT NULL,
    trade_date character varying(20) NOT NULL,
    close numeric,
    open numeric,
    high numeric,
    low numeric,
    pre_close numeric,
    avg_price numeric,
    change numeric,
    pct_change numeric,
    vol numeric,
    turnover_rate numeric,
    total_mv numeric,
    float_mv numeric
);


--
-- TOC entry 225 (class 1259 OID 24693)
-- Name: ths_index; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.ths_index (
    ts_code character varying(20) NOT NULL,
    name character varying(100),
    count integer,
    exchange character varying(10) NOT NULL,
    list_date character varying(20) NOT NULL,
    type character varying(20) NOT NULL
);


--
-- TOC entry 3423 (class 0 OID 0)
-- Dependencies: 225
-- Name: TABLE ths_index; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.ths_index IS '同花顺概念和行业指数';


--
-- TOC entry 226 (class 1259 OID 24696)
-- Name: ths_member; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.ths_member (
    ts_code character varying(20) NOT NULL,
    con_code character varying(20) NOT NULL,
    con_name character varying(100),
    weight numeric,
    in_date character varying(20),
    out_date character varying(20),
    is_new character varying(1)
);


--
-- TOC entry 227 (class 1259 OID 24701)
-- Name: trade_calendar; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.trade_calendar (
    exchange character varying(200) NOT NULL,
    cal_date character varying(200) NOT NULL,
    is_open smallint NOT NULL,
    pretrade_date character varying(200)
);


--
-- TOC entry 3250 (class 2606 OID 24707)
-- Name: balancesheet balancesheet_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.balancesheet
    ADD CONSTRAINT balancesheet_pk PRIMARY KEY (ts_code, ann_date, f_ann_date, end_date);


--
-- TOC entry 3252 (class 2606 OID 24709)
-- Name: dc_index dc_index_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.dc_index
    ADD CONSTRAINT dc_index_pk PRIMARY KEY (ts_code, trade_date);


--
-- TOC entry 3254 (class 2606 OID 24711)
-- Name: margin margin_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.margin
    ADD CONSTRAINT margin_pk PRIMARY KEY (trade_date, exchange_id);


--
-- TOC entry 3256 (class 2606 OID 24713)
-- Name: stk_holdertrade stk_holdertrade_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.stk_holdertrade
    ADD CONSTRAINT stk_holdertrade_pk PRIMARY KEY (ts_code, ann_date, holder_name, in_de);


--
-- TOC entry 3262 (class 2606 OID 24715)
-- Name: stock_daily_basic stock_daily_basic_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.stock_daily_basic
    ADD CONSTRAINT stock_daily_basic_pk PRIMARY KEY (ts_code, trade_date);


--
-- TOC entry 3260 (class 2606 OID 24717)
-- Name: stock_daily stock_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.stock_daily
    ADD CONSTRAINT stock_pk PRIMARY KEY (ts_code, trade_date);


--
-- TOC entry 3258 (class 2606 OID 24719)
-- Name: stock stock_pk1; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.stock
    ADD CONSTRAINT stock_pk1 PRIMARY KEY (ts_code);


--
-- TOC entry 3264 (class 2606 OID 24721)
-- Name: ths_daily ths_daily_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ths_daily
    ADD CONSTRAINT ths_daily_pk PRIMARY KEY (ts_code, trade_date);


--
-- TOC entry 3266 (class 2606 OID 24723)
-- Name: ths_index ths_index_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ths_index
    ADD CONSTRAINT ths_index_pk PRIMARY KEY (ts_code, exchange, type, list_date);


--
-- TOC entry 3268 (class 2606 OID 24725)
-- Name: ths_member ths_member_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ths_member
    ADD CONSTRAINT ths_member_pk PRIMARY KEY (ts_code, con_code);


--
-- TOC entry 3270 (class 2606 OID 24727)
-- Name: trade_calendar trade_calendar_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.trade_calendar
    ADD CONSTRAINT trade_calendar_pk PRIMARY KEY (cal_date, exchange);


-- Completed on 2025-07-28 17:16:27

--
-- PostgreSQL database dump complete
--

